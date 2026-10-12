//! The base sound: the fields and the verified behaviour of `rage::audSound`.
//!
//! A sound holds a voice column and a link, a kind word, a few flag bytes,
//! a sub-id, a voice id and option, an identity word, an attached voice,
//! eight slot ids, a sequence word and two stored slot pointers. The
//! original keeps them at fixed byte offsets of one object; here each is a
//! named field. Offsets appear in the field docs only, for a reader who
//! compares with the original's layout.
//!
//! The virtual table of the base class has nine slots. Their meaning, as
//! established from the verified rewrites and the table itself:
//!
//! | Slot | Method here |
//! |---|---|
//! | 0 | lookup by hash (`find_by_hash`), called through the object's table |
//! | 1 | lookup by name (`find_by_name`): hashes the name, then slot 0 |
//! | 2 | store through the found slot (`store_word_at_key`) |
//! | 3 | hash the key, then slot 2 (`hash_and_forward`) |
//! | 4 | forward the argument to a link target's slot 4 (`forward_link`) |
//! | 5 | deleting destructor (`deleting_destructor`) |
//! | 6 | always answers false (not lifted: no behaviour in it) |
//! | 7 | attach a voice description (attach_voice) |
//! | 8 | advance the voice record by one tick (advance) |
//!
//! Each lifted method is proven against its verified 32-bit rewrite with the
//! `lf-soundbase-diff` test crate; `registry` says what each proof narrows.

use lf_core::Handle32;

use super::world::{ObjectTag, SlotTag, SoundWorld};

/// The voice column or voice id that means "no voice".
pub const NO_VOICE: u8 = 0xff;
/// Eight slot ids per sound.
pub const SLOT_COUNT: usize = 8;
/// Bytes per voice-table row; the table is indexed by voice id.
pub const ROW_STRIDE: u32 = 0x6f40;
/// Row bias of the words the lookup and the forwarding method read.
pub const ROW_BIAS_LOOKUP: u32 = 0x6f10;
/// Row bias of the word the second constructor reads.
pub const ROW_BIAS_INIT: u32 = 0x6f14;
/// Fast-path bit in the first flag byte (`fast_flags`).
pub const FAST_BIT: u8 = 0x80;
/// The kind word value that, with the fast-path bit, disables lookups.
pub const KIND_FAST: u16 = 2;
/// Disable bit in the init flag byte: lookups answer nothing.
pub const DISABLE_BIT: u8 = 0x04;
/// Set in the init flag byte once the second constructor has run.
pub const INIT_DONE: u8 = 0x10;
/// Set in the init flag byte when the voice registration is still due.
pub const NEEDS_INIT: u8 = 0x02;
/// The sequence word after construction (minus one).
pub const SEQ_UNSET: u32 = 0xffff_ffff;

/// The fields of a base sound that the verified methods read or write.
///
/// Fields none of the lifted methods reads or writes are left out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AudSound {
    /// Voice column (byte at offset 0x04); `NO_VOICE` when unassigned.
    pub voice_column: u8,
    /// Link byte (byte at offset 0x05): the link target's row for the
    /// forwarding method, and the second selector for attach. `NO_VOICE`
    /// forwards to the fallback (forwarding) or takes the first branch (attach).
    pub link: u8,
    /// Kind word (16 bits at offset 0x06).
    pub kind: u16,
    /// First flag byte (offset 0x39): bit `FAST_BIT` enables the fast path.
    pub fast_flags: u8,
    /// Init flag byte (offset 0x3a): `INIT_DONE`, `NEEDS_INIT`, `DISABLE_BIT`.
    pub init_flags: u8,
    /// Sub-id byte (offset 0x3b): selects the init entry.
    pub sub_id: u8,
    /// Voice id (byte at offset 0x40): the voice-table row.
    pub voice_id: u8,
    /// Option byte (offset 0x41): the low six bits are the attach state.
    pub option: u8,
    /// Identity word (offset 0x3c), copied from the attach descriptor.
    pub identity: u32,
    /// Attached voice record (offset 0x98), set by attach.
    pub voice: u32,
    /// Slot ids (bytes at offsets 0x48..0x50); `NO_VOICE` marks an empty slot.
    pub slot_ids: [u8; SLOT_COUNT],
    /// Sequence word (offset 0xa4), set to `SEQ_UNSET` by construction.
    pub sequence: u32,
    /// First stored slot pointer (offset 0xa8): cleared as one byte.
    pub slot_a: u32,
    /// Second stored slot pointer (offset 0xac): cleared as one word.
    pub slot_b: u32,
}

impl AudSound {
    /// Looks a voice object up by hash: scans the slot ids and returns the
    /// first non-zero answer of the object's slot 0.
    ///
    /// Returns zero at once when the fast path applies (the fast bit set
    /// and the kind word equal to `KIND_FAST`) or when the disable bit is
    /// set. Otherwise each non-empty slot id forms a cursor from the
    /// scalar global and the voice table's row word; a zero cursor is
    /// skipped, and the first object whose slot 0 answers non-zero wins.
    #[must_use]
    pub fn find_by_hash(&self, world: &mut impl SoundWorld, hash: u32) -> u32 {
        if self.fast_flags & FAST_BIT != 0 && self.kind == KIND_FAST {
            return 0;
        }
        if self.init_flags & DISABLE_BIT != 0 {
            return 0;
        }
        let scalar = world.scalar_a();
        let row = world.voice_table_word(self.voice_id, ROW_BIAS_LOOKUP);
        for id in self.slot_ids {
            if id == NO_VOICE {
                continue;
            }
            let cursor = scalar.wrapping_mul(u32::from(id)).wrapping_add(row);
            if let Some(object) = Handle32::<ObjectTag>::new(cursor) {
                let found = world.find_slot0(object, hash);
                if found != 0 {
                    return found;
                }
            }
        }
        0
    }

    /// Finds an object by name: hashes the name with a zero seed, then
    /// calls the object's slot 0 with the hash and returns its answer.
    ///
    /// An associated function: the method reads no field of the sound; it
    /// only dispatches through the object it is given.
    #[must_use]
    pub fn find_by_name(
        this: Handle32<ObjectTag>,
        name: u32,
        world: &mut impl SoundWorld,
    ) -> u32 {
        let hash = world.joaat(name, 0);
        world.find_slot0(this, hash)
    }

    /// Looks the key up through slot 0 and, when it answers a slot, stores
    /// the float bits there. Returns the slot answer.
    ///
    /// An associated function: it reads no field of the sound.
    #[must_use]
    pub fn store_word_at_key(
        this: Handle32<ObjectTag>,
        key: u32,
        value_bits: u32,
        world: &mut impl SoundWorld,
    ) -> u32 {
        let slot = world.find_slot0(this, key);
        if let Some(slot) = Handle32::<SlotTag>::new(slot) {
            world.store_word(slot, value_bits);
        }
        slot
    }

    /// Hashes the key with a zero seed, then calls the object's slot 2 with
    /// the object, the hash and the float bits, and returns its answer.
    ///
    /// An associated function: it reads no field of the sound.
    #[must_use]
    pub fn hash_and_forward(
        this: Handle32<ObjectTag>,
        key: u32,
        value_bits: u32,
        world: &mut impl SoundWorld,
    ) -> u32 {
        let hashed = world.joaat(key, 0);
        world.slot2(this, hashed, value_bits)
    }

    /// Forwards a non-zero argument through the link: a link of `NO_VOICE`
    /// goes to the fallback cell's callee; any other link resolves a target
    /// from the scalar global, the link and the voice table's row word, and
    /// the target's slot 4 answers. A zero argument answers zero at once.
    #[must_use]
    pub fn forward_link(&self, arg: u32, world: &mut impl SoundWorld) -> u32 {
        if arg == 0 {
            return 0;
        }
        if self.link == NO_VOICE {
            let cell = world.fallback_cell();
            return world.forward_fallback(cell, arg);
        }
        let scalar = world.scalar_a();
        let entry = world.voice_table_word(self.voice_id, ROW_BIAS_LOOKUP);
        let target = scalar
            .wrapping_mul(u32::from(self.link))
            .wrapping_add(entry);
        world.slot4(target, arg)
    }

    /// The deleting destructor: runs the second constructor on the object,
    /// then, when bit 0 of `free` is set, unregisters the object with its
    /// voice id. Returns the object.
    ///
    /// The second constructor is a callee of this method in the lift (its
    /// effect on the fields is proven by [`AudSound::construct_second`]'s
    /// own proof). The voice id is read before the call: the second
    /// constructor never writes it.
    #[must_use]
    pub fn deleting_destructor(
        &self,
        this: Handle32<ObjectTag>,
        free: u32,
        world: &mut impl SoundWorld,
    ) -> Handle32<ObjectTag> {
        let voice_id = u32::from(self.voice_id);
        world.rebuild(this);
        if free & 1 != 0 {
            let registry = world.registry();
            world.unregister(registry, this, voice_id);
        }
        this
    }

    /// The second constructor: installs the class's virtual table, then runs
    /// one of two initialisation paths and resets the sequence word and the
    /// two stored slots.
    ///
    /// When the init-done bit is clear it is set and the init entry for the
    /// sub-id runs with the object. Otherwise, when the needs-init bit is
    /// also set and the voice column is assigned, a cursor is derived from
    /// the second scalar global and the voice table's init row word, and the
    /// cursor entry and the registry entry run with the voice id and kind.
    /// Finally the sequence word becomes `SEQ_UNSET`; a non-zero first slot
    /// pointer has one byte zeroed and is cleared, and a non-zero second
    /// slot pointer has one word zeroed and is cleared.
    pub fn construct_second(&mut self, this: Handle32<ObjectTag>, world: &mut impl SoundWorld) {
        world.install_vtable(this);
        let flags = self.init_flags;
        if flags & INIT_DONE == 0 {
            self.init_flags = flags | INIT_DONE;
            world.run_init(this, self.sub_id);
        } else if flags & NEEDS_INIT != 0 && self.voice_column != NO_VOICE {
            let voice_id = self.voice_id;
            let scalar = world.scalar_b();
            let word = world.voice_table_word(voice_id, ROW_BIAS_INIT);
            let cursor = scalar
                .wrapping_mul(u32::from(self.voice_column))
                .wrapping_add(word);
            world.set_cursor(cursor);
            let registry = world.registry();
            world.register(
                registry,
                u32::from(voice_id),
                u32::from(self.voice_column),
            );
        }
        self.sequence = SEQ_UNSET;
        if let Some(slot) = Handle32::<SlotTag>::new(self.slot_a) {
            world.zero_byte(slot);
            self.slot_a = 0;
        }
        if let Some(slot) = Handle32::<SlotTag>::new(self.slot_b) {
            world.zero_word(slot);
            self.slot_b = 0;
        }
    }

    /// Attaches a voice description: links the sound to the voice record,
    /// copies the identity word, the voice id and the option byte from the
    /// descriptor, runs three set-up callees over a scratch frame, then
    /// reconciles the sound's flags with the voice record. Returns 1, or 0
    /// when the descriptor or the voice is null, or when the first selector
    /// (`voice_column`) is unassigned in the first branch.
    ///
    /// Without a second selector (`link` is `NO_VOICE`), the first branch
    /// runs a reset callee, marks the voice record's flag byte, and, when
    /// the option's low six bits are all set, resolves a category through a
    /// memoised hash and a lookup callee and folds the lookup's verdict into
    /// the option. With a second selector, the second branch takes the
    /// option bits of the selected record that differ from the sound's.
    /// Last, the descriptor's flag byte is merged into the sound's first
    /// flag byte (bits 0 to 6 only; bit 7 is kept).
    ///
    /// The scratch frame is a pointer the set-up callees fill; this method
    /// never reads it, so the lift passes the world's frame handle and does
    /// not model what the callees write into it.
    #[must_use]
    pub fn attach_voice(
        &mut self,
        this: Handle32<ObjectTag>,
        voice: u32,
        desc: u32,
        extra: u32,
        world: &mut impl SoundWorld,
    ) -> u32 {
        if desc == 0 {
            return 0;
        }
        if voice == 0 {
            return 0;
        }
        self.voice = voice;
        self.identity = world.descriptor_word(desc);
        let pair = world.descriptor_pair(desc);
        self.voice_id = pair as u8;
        self.option = (pair >> 8) as u8;
        let frame = world.scratch_frame();
        world.init_block(frame);
        world.attach_callee(this, voice, 0, frame);
        world.bind_extra(this, extra, frame);
        self.sub_id = world.voice_sub_id(voice);
        if self.link == NO_VOICE {
            world.reset_object(this);
            if self.voice_column == NO_VOICE {
                return 0;
            }
            let row = self.voice_id;
            let stride = world.scalar_b();
            let word = world.voice_table_word(row, ROW_BIAS_INIT);
            let record = word.wrapping_add(u32::from(self.voice_column).wrapping_mul(stride));
            world.or_record_byte(record, RECORD_FLAGS, RECORD_FLAGS_MASK);
            if (self.option & OPTION_FULL) == OPTION_FULL {
                let mut cat = world.extra_word(extra);
                if cat == 0 {
                    let flag = world.memo_flag();
                    if flag & 1 == 0 {
                        world.set_memo_flag(flag | 1);
                        let h = world.category_key_hash();
                        world.set_memo_value(h);
                        cat = h;
                    } else {
                        cat = world.memo_value();
                    }
                }
                let found = world.category_lookup(cat);
                if found != 0 {
                    let ok = world.category_ok(found);
                    if (ok as u8) != 0 {
                        self.option = (self.option & 0xc1) | 1;
                    } else {
                        self.option &= 0xc0;
                    }
                }
            }
        } else {
            let row = self.voice_id;
            let stride = world.scalar_a();
            let word = world.voice_table_word(row, ROW_BIAS_LOOKUP);
            let record = word.wrapping_add(u32::from(self.link).wrapping_mul(stride));
            let diff = (world.record_byte(record, OPTION_FIELD) ^ self.option) & OPTION_FULL;
            self.option ^= diff;
        }
        let df = world.descriptor_flag(desc);
        self.fast_flags = ((df ^ self.fast_flags) & 0x7f) ^ df;
        1
    }

    /// Advances the sound's voice record by one tick: rotates the record's
    /// three history rows and tag rows by the lane's counter, and integrates
    /// a new velocity from the position history scaled by the reciprocal of
    /// the stamp delta. Returns the oldest tag word.
    ///
    /// The record is the voice column's row in the voice table. A lane
    /// selector (the record's byte at `LANE_SELECTOR_FIELD`, low three bits)
    /// picks the divisor-table entry that gives the counter. The lane's
    /// tick byte advances once per call. With no previous stamp (zero word
    /// at `STAMP_FIELD`) the velocity words are cleared; otherwise the
    /// velocity comes from the history difference, and a zero delta clears
    /// it. The reciprocal is taken of `delta * STEP_SCALE` in single
    /// precision, with the 32-bit delta read as signed plus 2^32 when its
    /// top bit is set.
    ///
    /// Narrowings, all from the verified rewrite: a sound with no voice column
    /// makes the original read a null record and fault, and this method
    /// panics at the same read (a trial the original faults on proves
    /// nothing). The divisor entries are taken as given; the rewrite has no
    /// timer wait for entries 0 to 2 (its contract pins them to 3 or more),
    /// so a port must establish that range itself. The rewrite writes zero to
    /// the record's word at `VELOCITY_PAD`, where the original reloads an
    /// uninitialised word below the stack: the lift writes zero too, as the
    /// rewrite defines it.
    #[must_use]
    pub fn advance(&self, stamp: u32, world: &mut impl SoundWorld) -> u32 {
        let voice = if self.voice_column == NO_VOICE {
            0
        } else {
            let stride = world.scalar_b();
            let word = world.voice_table_word(self.voice_id, ROW_BIAS_INIT);
            word.wrapping_add(u32::from(self.voice_column).wrapping_mul(stride))
        };
        assert!(
            voice != 0,
            "no voice column: the original reads a null voice record and faults"
        );
        let lane = u32::from(world.record_byte(voice, LANE_SELECTOR_FIELD) & 7);
        let count = world.divisor(lane, 0);
        let rot = count.wrapping_add(1) % 3;
        let rot2 = count.wrapping_add(2) % 3;
        let tick_at = count.wrapping_mul(32).wrapping_add(TICK_OFFSET);
        let tick = world.record_byte(voice, tick_at);
        world.set_record_byte(voice, tick_at, (tick & 0xe0) | (tick.wrapping_add(1) & 0x1f));
        let prev = world.record_word(voice, STAMP_FIELD);
        if prev == 0 {
            world.set_record_word(voice, 0, 0);
            world.set_record_word(voice, 4, 0);
            world.set_record_word(voice, 8, 0);
            world.set_record_word(voice, STAMP_FIELD, stamp);
        } else {
            let cur = count.wrapping_add(1).wrapping_mul(16);
            let reference = rot2.wrapping_add(1).wrapping_mul(16);
            let dv0 = f32::from_bits(world.record_word(voice, cur))
                - f32::from_bits(world.record_word(voice, reference));
            let dv1 = f32::from_bits(world.record_word(voice, cur + 4))
                - f32::from_bits(world.record_word(voice, reference + 4));
            let dv2 = f32::from_bits(world.record_word(voice, cur + 8))
                - f32::from_bits(world.record_word(voice, reference + 8));
            let dt = stamp.wrapping_sub(prev);
            world.set_record_word(voice, STAMP_FIELD, stamp);
            let tag = world.record_byte(voice, tick_at);
            world.set_record_byte(voice, tick_at, (tag & 0xe0) | (tag.wrapping_add(1) & 0x1f));
            if dt == 0 {
                world.set_record_word(voice, 0, 0);
                world.set_record_word(voice, 4, 0);
                world.set_record_word(voice, 8, 0);
            } else {
                let wide = f64::from(dt as i32)
                    + if dt >> 31 == 0 { 0.0 } else { 4_294_967_296.0 };
                let rate = 1.0 / ((wide as f32) * STEP_SCALE);
                world.set_record_word(voice, 0, (dv0 * rate).to_bits());
                world.set_record_word(voice, 4, (dv1 * rate).to_bits());
                world.set_record_word(voice, 8, (dv2 * rate).to_bits());
                world.set_record_word(voice, VELOCITY_PAD, 0.0f32.to_bits());
            }
        }
        let src_hist = voice.wrapping_add(count.wrapping_add(2).wrapping_mul(32));
        let dst_hist = voice.wrapping_add(rot.wrapping_add(2).wrapping_mul(32));
        for k in (0..32).step_by(4) {
            let w = world.record_word(src_hist, k);
            world.set_record_word(dst_hist, k, w);
        }
        let src_tag = voice.wrapping_add(count.wrapping_add(1).wrapping_mul(16));
        let dst_tag = voice.wrapping_add(rot.wrapping_add(1).wrapping_mul(16));
        let w0 = world.record_word(src_tag, 0);
        world.set_record_word(dst_tag, 0, w0);
        let w4 = world.record_word(src_tag, 4);
        world.set_record_word(dst_tag, 4, w4);
        let w8 = world.record_word(src_tag, 8);
        world.set_record_word(dst_tag, 8, w8);
        let tagw = world.record_word(src_tag, VELOCITY_PAD);
        world.set_record_word(dst_tag, VELOCITY_PAD, tagw);
        tagw
    }
}

/// Offset of the lane selector byte in a voice record (low three bits).
pub const LANE_SELECTOR_FIELD: u32 = 0xe7;
/// Offset of the lane tick byte base: the tick byte of counter `c` is at
/// `c * 32 + TICK_OFFSET`.
pub const TICK_OFFSET: u32 = 0x5f;
/// Offset of the stamp word of the voice record.
pub const STAMP_FIELD: u32 = 0xd0;
/// Offset of the word the velocity clears last, and the tag row's last word.
pub const VELOCITY_PAD: u32 = 0xc;
/// The single-precision step scale of the velocity integration (measured from
/// the executable's constant, as the rewrite reads it).
pub const STEP_SCALE: f32 = f32::from_bits(0x3d08_850a);

/// Offset of the flag byte of a voice record that attach marks.
pub const RECORD_FLAGS: u32 = 0xe7;
/// Bits attach ORs into that flag byte of the voice record.
pub const RECORD_FLAGS_MASK: u8 = 0x38;
/// Offset of the option byte in a voice record.
pub const OPTION_FIELD: u32 = 0x41;
/// Low six bits of the option byte: all set means full set-up.
pub const OPTION_FULL: u8 = 0x3f;
