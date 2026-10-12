//! The mirror-reflection render phase: its object and its four methods.
//!
//! The object keeps four words and a byte of its own: the target handle, a
//! handle to the parameter block, three filter seeds, and a status byte.
//! Everything else the methods reach is either a global (see
//! [`MirrorGlobals`]), the scene snapshot passed to the reflection method,
//! or an object owned by someone else, reached through [`MirrorHost`].
//!
//! The methods, by virtual slot of the phase: the deleting destructor (slot
//! 0), the refresh that rebuilds the reflection target (slot 3), the
//! reflection render over the snapshot (slot 4), and the render that draws
//! the phase (slot 8).

use core::hint::black_box;

use crate::mirror_phase::globals::MirrorGlobals;
use crate::mirror_phase::host::{MirrorHost, Ref, Word};

// Byte offsets inside the render-phase object.
/// The target handle, written by the refresh and read by the render.
const OFF_TARGET: u32 = 0x938;
/// The parameter block handle.
const OFF_PARAMS: u32 = 0x940;
/// The three filter seeds, one word each.
const OFF_SEEDS: [u32; 3] = [0x510, 0x514, 0x518];
/// The status byte, set by the reflection method.
const OFF_STATUS: u32 = 0x1D;
/// The descriptor the render method probes and binds.
const OFF_RENDER_DESC: u32 = 0xB0;
/// The second descriptor the reflection method binds.
const OFF_REFLECT_DESC_B: u32 = 0x4A0;
/// The three descriptors the refresh rebuilds the target from.
const OFF_REFRESH_DESC: [u32; 3] = [0x900, 0x8F8, 0x8FC];

// Byte offsets inside the parameter block (a separate object).
/// The selector word of the parameter block.
const PARAM_SELECTOR: u32 = 0x30;
/// The count word of the parameter block.
const PARAM_COUNT: u32 = 0x38;
/// The latched selector the reflection method writes into the block.
const PARAM_LATCH_SELECTOR: u32 = 0x78;
/// The latched count the reflection method writes into the block.
const PARAM_LATCH_COUNT: u32 = 0x7C;

// Byte offsets inside the scene snapshot.
/// The view the snapshot's descriptors are bound to.
const SNAP_VIEW: u32 = 0x10;
/// The selector of the snapshot.
const SNAP_SELECTOR: u32 = 0x538;
/// The status byte of the snapshot.
const SNAP_STATUS: u32 = 0x558;

/// Byte offset of the flag word inside a player or target object.
const OBJECT_FLAGS: u32 = 0x24;
/// The bit of the flag word the refresh remembers.
const FLAG_REMEMBERED_BIT: u32 = 5;

/// The target value meaning "no target".
const NO_TARGET: u32 = 0xFFFF_FFFF;
/// The first slot of the deleting destructor's free-bit flag.
const FREE_BIT: u32 = 1;
/// The selector the render method passes to select.
const RENDER_SELECT_MODE: u32 = 0x307;
/// The value the refresh passes as its first hook argument.
const HOOK_FIRST_ARG: u32 = 1;

/// The query kinds the render method asks for, by use.
const QUERY_KIND_BIND: u32 = 0x18;
const QUERY_KIND_FEED: u32 = 0x20;
const QUERY_KIND_TOGGLE: u32 = 0x0C;
const QUERY_KIND_PAIR: u32 = 0x10;
/// One, as float bits, used by the six-word setter.
const ONE_BITS: u32 = 0x3F80_0000;

/// The state of one mirror-reflection render phase.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MirrorReflectionPhase {
    /// The target handle.
    pub target: u32,
    /// The parameter block handle; zero when there is none.
    pub params: u32,
    /// The three filter seeds.
    pub seeds: [u32; 3],
    /// The status byte: bit 2 of the snapshot's status.
    pub status: u8,
}

impl MirrorReflectionPhase {
    /// Loads the phase's own words from its object, which holds them at the
    /// byte offsets above (little-endian words).
    #[must_use]
    pub fn load(object: &[u32]) -> Self {
        let status_word = object[(OFF_STATUS - 1) as usize / 4];
        Self {
            target: object[OFF_TARGET as usize / 4],
            params: object[OFF_PARAMS as usize / 4],
            seeds: [
                object[OFF_SEEDS[0] as usize / 4],
                object[OFF_SEEDS[1] as usize / 4],
                object[OFF_SEEDS[2] as usize / 4],
            ],
            status: ((status_word >> 8) & 0xFF) as u8,
        }
    }

    /// Stores the phase's own words back into its object, leaving every
    /// other word of it as it was.
    pub fn store(&self, object: &mut [u32]) {
        object[OFF_TARGET as usize / 4] = self.target;
        object[OFF_PARAMS as usize / 4] = self.params;
        for (offset, value) in OFF_SEEDS.iter().zip(self.seeds) {
            object[*offset as usize / 4] = value;
        }
        let index = (OFF_STATUS - 1) as usize / 4;
        object[index] = (object[index] & !0xFF00) | (u32::from(self.status) << 8);
    }

    /// The parameter block the phase must have to refresh or reflect.
    ///
    /// # Panics
    ///
    /// When the phase has no parameter block. The original reads a field of
    /// it on this path and faults, so the lift states the domain instead.
    fn param_block(&self) -> u32 {
        assert!(
            self.params != 0,
            "the original faults on a null parameter block; the lift requires one"
        );
        self.params
    }

    /// The deleting destructor: runs the object's destructor, then frees its
    /// storage when bit 0 of `flags` is set. The original returns the object
    /// it was given; the lift does not return it (the caller already holds it).
    pub fn deleting_destructor<H: MirrorHost>(host: &mut H, flags: u32) {
        host.destruct(Ref::Phase(0));
        if flags & FREE_BIT != 0 {
            host.free_storage(Ref::Phase(0));
        }
    }

    /// Renders the phase: gates, probes, a float block and a toggle, four
    /// query blocks (two stamped), a player call, a selector, and a final
    /// gated toggle. The original returns zero; so does the lift.
    pub fn render<H: MirrorHost>(&self, host: &mut H, g: &mut MirrorGlobals) {
        if g.render_enabled == 0 || g.render_skipped != 0 {
            return;
        }
        if self.target == NO_TARGET {
            return;
        }
        let probe = host.probe_gate(Ref::Phase(OFF_RENDER_DESC));
        // The staged float doubles as the high bytes of a later argument:
        // zero when the probe's low byte is set, otherwise one.
        let staged = if probe & 0xFF != 0 { 0 } else { g.one_bits };

        let q = host.query(QUERY_KIND_BIND, 0);
        let h = if q != 0 {
            host.apply_six(Ref::Handle(q), 0, 0, 1, staged, 1, 0)
        } else {
            0
        };
        host.release(Ref::Handle(h));

        if g.float_block_enabled != 0 {
            let floats = host.fetch_floats();
            let v0 = host.word(Ref::Handle(floats), 0);
            let v1 = host.word(Ref::Handle(floats), 4);
            let v2 = host.word(Ref::Handle(floats), 8);
            let v3 = mul(
                f32::from_bits(host.word(Ref::Handle(floats), 12)),
                f32::from_bits(g.scale_bits),
            );
            let q2 = host.query(QUERY_KIND_FEED, 0);
            let h2 = if q2 != 0 {
                let mut block = [v0, v1, v2, v3.to_bits()];
                host.feed_four(Ref::Handle(q2), 0, &mut block)
            } else {
                0
            };
            host.release(Ref::Handle(h2));
            let q3 = host.query(QUERY_KIND_TOGGLE, 0);
            let h3 = if q3 != 0 {
                host.toggle(Ref::Handle(q3), 1)
            } else {
                0
            };
            host.release(Ref::Handle(h3));
        }

        let q4 = host.query(QUERY_KIND_TOGGLE, 0);
        if q4 != 0 {
            stamp_block(host, g, q4, g.trailer_first);
        }
        host.release(Ref::Handle(q4));
        let q5 = host.query(QUERY_KIND_TOGGLE, 0);
        if q5 != 0 {
            stamp_block(host, g, q5, g.trailer_second);
        }
        host.release(Ref::Handle(q5));
        let q6 = host.query(QUERY_KIND_PAIR, 0);
        let h6 = if q6 != 0 {
            host.pair(Ref::Handle(q6), 2, 0)
        } else {
            0
        };
        host.release(Ref::Handle(h6));
        let q7 = host.query(QUERY_KIND_PAIR, 0);
        let h7 = if q7 != 0 {
            host.pair(Ref::Handle(q7), 8, 1)
        } else {
            0
        };
        host.release(Ref::Handle(h7));

        let mut player_for_call = 0;
        let mut flag = 0;
        let found = host.find_player(0);
        if found != 0 {
            flag = (host.word(Ref::Handle(found), OBJECT_FLAGS) >> FLAG_REMEMBERED_BIT) & 1;
            host.slot30(Ref::Handle(found), Word::Value(1));
            player_for_call = found;
        }
        let selector = if g.select_flag_a != 0 || g.select_flag_b != 0 {
            1
        } else {
            2
        };
        host.select(Ref::Handle(self.target), RENDER_SELECT_MODE, selector);
        if player_for_call != 0 {
            // The original's pushed word was filled with the staged float and
            // then overwritten byte-wise with the flag bit; the lift holds the
            // two parts and rebuilds that word from them.
            host.slot30(
                Ref::Handle(player_for_call),
                Word::Value((staged & 0xFFFF_FF00) | flag),
            );
        }
        host.commit(Ref::Handle(self.target));
        if g.float_block_enabled != 0 {
            let q8 = host.query(QUERY_KIND_TOGGLE, 0);
            let h8 = if q8 != 0 {
                host.toggle(Ref::Handle(q8), 0)
            } else {
                0
            };
            host.release(Ref::Handle(h8));
        }
    }

    /// Refreshes the reflection target: gates and two parameter checks, a
    /// fetch and rebuild of the target, a hook on the current target object,
    /// a temporary swap of the parameter block from the global table around
    /// a commit, the second hook, then the footer's answer. The original's
    /// early returns answer its accumulator; the lift answers zero there.
    pub fn refresh<H: MirrorHost>(&mut self, host: &mut H, g: &MirrorGlobals) -> u32 {
        if g.render_enabled == 0 || g.render_skipped != 0 {
            return 0;
        }
        let params = self.param_block();
        if host.word(Ref::Handle(params), PARAM_SELECTOR) == 0 {
            return 0;
        }
        if (host.word(Ref::Handle(params), PARAM_COUNT) as i32) <= 0 {
            return 0;
        }

        self.target = host.fetch_target(Ref::Handle(g.session));
        host.rebuild_target(
            Ref::Phase(OFF_REFRESH_DESC[0]),
            Ref::Phase(OFF_REFRESH_DESC[1]),
            Ref::Phase(OFF_REFRESH_DESC[2]),
            Ref::Phase(0),
            1,
            0,
        );
        let object = host.lookup_object(0);
        // The remembered word: the identity with the bit of the current
        // object's flags, or one when there is no object.
        let mut remembered = Word::PhaseTagged(1);
        if object != 0 {
            let bit = (host.word(Ref::Handle(object), OBJECT_FLAGS) >> FLAG_REMEMBERED_BIT) & 1;
            remembered = Word::PhaseTagged(bit as u8);
            host.slot30(Ref::Handle(object), Word::Value(HOOK_FIRST_ARG));
        }

        let mut saved_selector = 0;
        let mut saved_count = 0;
        if g.swap_enabled != 0 {
            let block = self.params;
            if block != 0 {
                let entry = g.swap_index.wrapping_mul(3);
                let (count, selector) = table_entry(&g.table, entry);
                saved_count = host.word(Ref::Handle(block), PARAM_COUNT);
                saved_selector = host.word(Ref::Handle(block), PARAM_SELECTOR);
                host.set_word(Ref::Handle(block), PARAM_COUNT, count);
                host.set_word(Ref::Handle(self.params), PARAM_SELECTOR, selector);
            }
        }

        host.commit(Ref::Handle(self.target));
        if self.params != 0 && saved_selector != 0 {
            host.set_word(Ref::Handle(self.params), PARAM_COUNT, saved_count);
            host.set_word(Ref::Handle(self.params), PARAM_SELECTOR, saved_selector);
        }

        if object != 0 {
            host.slot30(Ref::Handle(object), remembered);
        }
        host.footer()
    }

    /// Reflects the scene snapshot: binds both descriptors, reduces the
    /// parameter block into a frame seeded with the phase's three seeds,
    /// publishes the block's selector and count, latches them into the
    /// block when it is usable, runs the four-stage chain (or the fallback),
    /// projects the current matrix and a scaled fresh one, and copies the
    /// snapshot's status bit into the phase.
    ///
    /// The original returns the snapshot pointer with its low byte replaced
    /// by that bit; the lift returns the bit alone.
    pub fn reflect<H: MirrorHost>(&mut self, host: &mut H, g: &mut MirrorGlobals) -> u8 {
        host.bind(Ref::Phase(OFF_RENDER_DESC), Ref::Snapshot(SNAP_VIEW));
        host.bind(Ref::Phase(OFF_REFLECT_DESC_B), Ref::Snapshot(SNAP_VIEW));

        let params = self.params;
        // Frame: three seeds, then zeros. The reducer fills words four to
        // seven of it on the original's side; the lift's host does the same.
        let mut frame = [self.seeds[0], self.seeds[1], self.seeds[2], 0, 0, 0, 0, 0];
        if params != 0 {
            let snapshot_selector = host.word(Ref::Snapshot(0), SNAP_SELECTOR);
            host.reduce(Ref::Handle(params), snapshot_selector, &mut frame);
            g.published_selector = host.word(Ref::Handle(params), PARAM_SELECTOR);
            g.published_count = host.word(Ref::Handle(params), PARAM_COUNT);
        }

        let block = self.params;
        if block != 0
            && host.word(Ref::Handle(block), PARAM_SELECTOR) != 0
            && (host.word(Ref::Handle(block), PARAM_COUNT) as i32) > 0
        {
            let count = host.word(Ref::Handle(block), PARAM_COUNT);
            host.set_word(Ref::Handle(block), PARAM_LATCH_COUNT, count);
            let fresh = self.params;
            let fresh_selector = host.word(Ref::Handle(fresh), PARAM_SELECTOR);
            host.set_word(Ref::Handle(fresh), PARAM_LATCH_SELECTOR, fresh_selector);

            let half: &mut [u32; 4] = (&mut frame[4..8])
                .try_into()
                .expect("the frame's second half is four words");
            host.describe(half);
            host.describe_with(half, Ref::Phase(OFF_RENDER_DESC));
            let selector = host.word(Ref::Handle(block), PARAM_SELECTOR);
            host.combine(selector, Ref::Handle(block), half, Ref::Phase(0), 0);
            host.finish(half);
        } else {
            host.fallback(0);
        }

        let matrix = host.current_matrix();
        let mut projected = matrix;
        host.project(Ref::Phase(0), &mut projected);
        host.set_six(Ref::Phase(OFF_RENDER_DESC), 0, 0, ONE_BITS, ONE_BITS, 0, ONE_BITS);

        if g.scale_enabled != 0 {
            let fresh = host.fresh_matrix();
            let scaled = mul(f32::from_bits(fresh[3]), f32::from_bits(g.scale_bits));
            let mut scaled_matrix = [fresh[0], fresh[1], fresh[2], scaled.to_bits()];
            host.scale_project(Ref::Phase(OFF_RENDER_DESC), &mut scaled_matrix);
        }

        let source = host.byte(Ref::Snapshot(0), SNAP_STATUS);
        let status = (source >> 2) & 1;
        self.status = status;
        status
    }
}

/// Stamps one queried block: writes the first tag, mixes the block's second
/// word with the stamp counter, advances the counter, writes the final tag
/// and the trailer.
fn stamp_block<H: MirrorHost>(host: &mut H, g: &mut MirrorGlobals, block: u32, trailer: u32) {
    let object = Ref::Handle(block);
    let field = host.word(object, 4);
    host.set_word(object, 0, g.stamp_tag_first);
    let counter = g.stamp_counter;
    let mix = (field ^ counter) & 0x3FFF;
    host.set_word(object, 4, field ^ mix);
    g.stamp_counter = counter.wrapping_add(1);
    host.set_word(object, 0, g.stamp_tag_final);
    host.set_word(object, 8, trailer);
}

/// Reads the count and selector at a parameter-table entry: the entry is
/// three words, the count first and the selector second.
///
/// # Panics
///
/// When the entry is outside the table. The original reads the image there;
/// the lift states the domain instead of reading past the table.
fn table_entry(table: &[u32], entry: u32) -> (u32, u32) {
    let index = entry as usize;
    assert!(
        index + 1 < table.len(),
        "parameter table entry outside the table the image holds"
    );
    (table[index], table[index + 1])
}

/// Float multiply with both operands pinned, so the result carries the first
/// operand's payload when both are not-a-number.
fn mul(a: f32, b: f32) -> f32 {
    black_box(a) * black_box(b)
}
