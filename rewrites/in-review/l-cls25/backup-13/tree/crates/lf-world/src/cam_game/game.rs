//! The game camera object and its lifted methods.
//!
//! Field names say what each word is used for where that is established by
//! the verified rewrites; words whose use is not established keep their
//! offset in the name and are marked in the field docs. Bit masks name the
//! marker each method sets or clears.

use lf_core::Handle32;

use super::world::{
    CamGameWorld, CamGlobal, CamOwnerTag, CamTag, GateBlock, GateTag, SingletonField, SingletonTag,
};

/// Mode word an idle camera holds (set by the idle reset).
pub const IDLE_MODE: u32 = 3;

/// Timer word an idle camera holds (set by the idle reset).
pub const IDLE_TIMER: u32 = 0x7530;

/// Marker bit of the first flag byte that the idle reset sets.
pub const IDLE_MARKER: u8 = 0x80;

/// Marker bit of the first flag byte that the restart clears.
pub const ACTIVE_MARKER: u8 = 0x40;

/// The zoom an idle camera holds, as a float.
pub const DEFAULT_ZOOM: f32 = 1.0;

/// Worker mode the restart asks for.
const WORKER_MODE: u32 = 1;

/// Extra word the restart passes when it asks for a worker.
const WORKER_EXTRA: u32 = 0;

/// Low flag bit of the deleting destructor: return the storage to the pool.
const RELEASE_TO_POOL: u32 = 1;

/// The camera state the three lifted methods read or write.
///
/// The comments give each field's offset within the original object, which
/// is the only place an address appears; the lifted methods never see one.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CCamGame {
    /// Blend channel, as the bits of a float (offset 0x74). The blend step raises,
    /// clamps and zeroes it. Stored as bits so that NaN payloads survive.
    pub blend: u32,
    /// The owner the camera is attached to (offset 0x114). `None` for zero.
    pub owner: Option<Handle32<CamOwnerTag>>,
    /// Mode word (offset 0x140). The idle reset sets it to [`IDLE_MODE`].
    pub mode: u32,
    /// Byte beside the mode word (offset 0x144). Cleared by the idle reset.
    pub mode_flag: u8,
    /// Default word copied from the shared global (offset 0x148).
    pub default_word: u32,
    /// Timer word (offset 0x14C). The idle reset sets it to [`IDLE_TIMER`].
    pub timer: u32,
    /// Byte beside the timer word (offset 0x150). Cleared by the idle reset.
    pub timer_flag: u8,
    /// Word cleared by the idle reset (offset 0x154). Use not established.
    pub word_154: u32,
    /// Zoom, as the bits of a float (offset 0x1BC). The idle reset sets 1.0.
    pub zoom_bits: u32,
    /// First flag byte (offset 0x1C4). The idle reset clears its low three
    /// bits before the reset step and bits 3 to 5 after it, then sets
    /// [`IDLE_MARKER`]; the restart clears [`ACTIVE_MARKER`].
    pub flags_1c4: u8,
    /// Second flag byte (offset 0x1C5). The idle reset clears bits 1 to 3
    /// before the reset step and bit 0 after it.
    pub flags_1c5: u8,
    /// Pending word (offset 0x1C8), cleared by the restart. Use not established.
    pub pending_1c8: u32,
    /// Pending word (offset 0x1CC), cleared by the restart. Use not established.
    pub pending_1cc: u32,
    /// Pending word (offset 0x1D0), cleared by the restart. Use not established.
    pub pending_1d0: u32,
    /// Word cleared by the idle reset (offset 0x1D4). Use not established.
    pub word_1d4: u32,
    /// Byte cleared by the idle reset (offset 0x1D8). Use not established.
    pub flag_1d8: u8,
    /// Word cleared by the idle reset (offset 0x1DC). Use not established.
    pub word_1dc: u32,
    /// Word cleared by the idle reset (offset 0x1E0). Use not established.
    pub word_1e0: u32,
    /// Word cleared by the idle reset (offset 0x1E4). Use not established.
    pub word_1e4: u32,
    /// Word cleared by the idle reset (offset 0x1E8). Use not established.
    pub word_1e8: u32,
}

impl CCamGame {
    /// Deleting destructor.
    ///
    /// Runs the class's destructor body first; then, when the low bit of
    /// `flags` is set, asks the pool manager to take the camera's storage
    /// back. The original's return value (the object's own identity) is not
    /// modelled: the owner already holds `this`.
    pub fn deleting_destructor(
        &mut self,
        world: &mut impl CamGameWorld,
        this: Handle32<CamTag>,
        flags: u32,
    ) {
        world.destroy_body(self, this);
        if flags & RELEASE_TO_POOL != 0 {
            let manager = world.pool_manager();
            world.pool_release(manager, this);
        }
    }

    /// Restart.
    ///
    /// Runs the camera's own teardown step, builds a worker for the owner,
    /// chains the worker's flags, releases the worker, then clears the
    /// active marker and the pending words. The original always answers 1;
    /// that constant is not returned.
    pub fn restart(&mut self, world: &mut impl CamGameWorld, this: Handle32<CamTag>) {
        world.teardown(self, this);
        let owner = self.owner;
        let worker = world.create_worker(owner, WORKER_MODE, WORKER_EXTRA, this);
        world.chain_set_flags(worker);
        world.release_worker(worker);
        self.flags_1c4 &= !ACTIVE_MARKER;
        self.pending_1c8 = 0;
        self.pending_1cc = 0;
        self.pending_1d0 = 0;
    }

    /// Idle reset.
    ///
    /// Clears the mode, timer and flag words, copies the shared default word
    /// into its slot and runs the shared reset step. The reset step may
    /// change the flag byte, so the byte is read again after it: bits 3 to 5
    /// are cleared from whatever the step left, and the idle marker is set.
    /// Then it sets the default zoom and clears the remaining words. The
    /// original always answers 1; that constant is not returned.
    pub fn reset_to_idle(&mut self, world: &mut impl CamGameWorld, this: Handle32<CamTag>) {
        self.flags_1c5 &= 0xf1;
        self.word_1e4 = 0;
        self.word_1e8 = 0;
        self.word_1e0 = 0;
        self.mode_flag = 0;
        self.mode = IDLE_MODE;
        self.timer_flag = 0;
        self.flags_1c4 &= 0xf8;
        self.default_word = world.shared_default_word();
        self.timer = IDLE_TIMER;
        world.reset_state_block(self, this);
        let flags = self.flags_1c4;
        self.flags_1c5 &= 0xfe;
        self.flags_1c4 = (flags & 0xc7) | IDLE_MARKER;
        self.zoom_bits = DEFAULT_ZOOM.to_bits();
        self.flag_1d8 = 0;
        self.word_1d4 = 0;
        self.word_154 = 0;
        self.word_1dc = 0;
    }
}

/// Offset of the selector byte in a gate status object.
const GATE_SELECTOR: u32 = 0x328D;
/// Offset of the first base byte of a gate status object.
const GATE_BASE: u32 = 0x269C;
/// Offset of the first compared byte of a gate status object.
const GATE_FIRST: u32 = 0x269E;
/// Offset of the second compared byte of a gate status object.
const GATE_SECOND: u32 = 0x269F;
/// Offset of the first byte of the pair that tests the notify condition.
const GATE_PAIR_LOW: u32 = 0x29BC;
/// Offset of the second byte of the pair that tests the notify condition.
const GATE_PAIR_HIGH: u32 = 0x29BE;

/// Threshold of the xor-folded byte tests: a folded value above it is "high".
const HIGH_THRESHOLD: u8 = 0x7F;

/// The busy bit of the tail object's flag byte, cleared by the tail step.
const TAIL_BUSY: u8 = 0x20;

/// The state the tail object must hold for the matched verdict.
const TAIL_MATCHED_STATE: u8 = 3;

/// First and last middle-chain step; the chain runs each step in order.
const MIDDLE_FIRST: u32 = 6;
const MIDDLE_LAST: u32 = 12;

/// The one-bit verdict of a gate block, from the gate status object's bytes.
///
/// When the selector byte is nonzero the verdict is set only when the first
/// folded pair is high and the second is not. When the selector is zero the
/// verdict needs the first pair low, the second high, and the inhibit flag
/// clear. The folded pairs are the base byte xor a compared byte.
fn gate_verdict(world: &mut impl CamGameWorld, gate: Handle32<GateTag>, inhibit: u8) -> bool {
    if world.gate_byte(gate, GATE_SELECTOR) != 0 {
        let base = world.gate_byte(gate, GATE_BASE);
        if world.gate_byte(gate, GATE_FIRST) ^ base > HIGH_THRESHOLD
            && world.gate_byte(gate, GATE_SECOND) ^ base <= HIGH_THRESHOLD
        {
            return true;
        }
        false
    } else {
        let base = world.gate_byte(gate, GATE_BASE);
        if world.gate_byte(gate, GATE_FIRST) ^ base > HIGH_THRESHOLD {
            return false;
        }
        if world.gate_byte(gate, GATE_SECOND) ^ base <= HIGH_THRESHOLD {
            return false;
        }
        inhibit == 0
    }
}

impl CCamGame {
    /// Camera event step: two gate blocks around a middle update chain, then
    /// the tail.
    ///
    /// Each gate block asks the provider for a gate status object and, when
    /// the verdict on it is set and its pair test is high, asks the watcher
    /// for a target and notifies the sink with the global word of its block.
    /// The middle chain runs one sync step, then, while the singleton
    /// exists, seven update steps. The tail asks the watcher once more; when
    /// it answers, the matched byte records whether the tail's state is the
    /// matched state, the busy bit is cleared and the state is zeroed. The
    /// done flag is cleared last. The original always answers 1; that
    /// constant is not returned.
    pub fn gate_update(&mut self, world: &mut impl CamGameWorld, this: Handle32<CamTag>) {
        let inhibit = world.inhibit_flag();
        self.gate_block(world, this, GateBlock::First, inhibit);
        world.sync_target(self, this);
        if world.singleton_present() {
            for step in MIDDLE_FIRST..=MIDDLE_LAST {
                world.middle_update(self, this, step);
            }
        }
        self.gate_block(world, this, GateBlock::Second, inhibit);
        if let Some(tail) = world.tail(self, this) {
            let matched = world.tail_state(tail) == TAIL_MATCHED_STATE;
            world.set_matched(u8::from(matched));
            let flags = world.tail_flags(tail);
            if flags & TAIL_BUSY != 0 {
                world.set_tail_flags(tail, flags & !TAIL_BUSY);
            }
            world.set_tail_state(tail, 0);
        }
        world.clear_done();
    }

    /// One gate block of [`CCamGame::gate_update`].
    fn gate_block(
        &mut self,
        world: &mut impl CamGameWorld,
        this: Handle32<CamTag>,
        block: GateBlock,
        inhibit: u8,
    ) {
        let Some(gate) = world.gate_status(block) else {
            return;
        };
        if !gate_verdict(world, gate, inhibit) {
            return;
        }
        let pair_low = world.gate_byte(gate, GATE_PAIR_LOW);
        let pair_high = world.gate_byte(gate, GATE_PAIR_HIGH);
        if pair_high ^ pair_low <= HIGH_THRESHOLD {
            return;
        }
        if world.find_target(self, this, block).is_some() {
            let word = world.gate_word(block);
            world.notify_sink(self, this, word, block);
        }
    }
}
/// Mask of the first flag byte that marks the camera as initialised.
const FLAG_INITIALISED: u8 = 0x80;

/// Mask of the first flag byte that forces the blend channel to zero.
const FLAG_ZERO_BLEND: u8 = 0x04;

/// Sum with the operands kept in order, each passed through an opaque
/// barrier so the compiler cannot swap them: the payload of a NaN result
/// follows the first operand.
fn ordered_add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

/// The packed counter step. The original's multiply-and-shift sequence leaves
/// the remainder of the count, read as a signed value, after division by
/// three: `(n as i32) % 3` (checked on the host for every count up to
/// 200000 and for the negative words). The sequence is kept as the original
/// runs it, with the same wrapping at 32 bits.
fn counter_step(n: u32) -> u32 {
    let product = 0x5555_5555_i64 * i64::from(n as i32);
    let mut quotient = (product >> 32) as i32;
    quotient = quotient.wrapping_sub(n as i32);
    quotient >>= 1;
    let correction = (((quotient as u32) >> 31) as i32).wrapping_add(quotient);
    let step = correction as u32;
    n.wrapping_add(step).wrapping_add(step.wrapping_mul(2))
}

/// The float a global holds, from its raw bits.
fn float_global(world: &mut impl CamGameWorld, global: CamGlobal) -> f32 {
    f32::from_bits(world.read_global(global))
}

impl CCamGame {
    /// Camera update step: four update callees, then the initialised marker
    /// (set on the first run, otherwise the watcher is notified). While the
    /// singleton exists, the two blend channels advance; then two more
    /// update callees run and the blend is zeroed when the zero flag is set.
    /// The original always answers 1; that constant is not returned.
    pub fn update_step(&mut self, world: &mut impl CamGameWorld, this: Handle32<CamTag>) {
        world.update_1(self, this);
        world.update_2(self, this);
        world.update_3(self, this);
        world.update_4(self, this);
        if self.flags_1c4 & FLAG_INITIALISED == 0 {
            self.flags_1c4 |= FLAG_INITIALISED;
        } else {
            world.watch_init(self, this);
        }
        if let Some(singleton) = world.singleton() {
            self.blend_channel_one(world, singleton);
            self.blend_channel_two(world, singleton);
            world.update_9(self, this);
        }
        world.update_10(self, this);
        world.update_11(self, this);
        if self.flags_1c4 & FLAG_ZERO_BLEND != 0 {
            self.blend = 0;
        }
    }

    /// First blend channel: the step is added to the blend, the cap raises
    /// it, and it is clamped to the ceiling on the upper side only when
    /// positive; NaN passes through. The first accumulator then advances
    /// unless it is cleared, and the counter moves when the accumulator is
    /// exactly zero. An inhibited or inactive singleton clears the
    /// accumulator instead, unless the fallback byte is set.
    fn blend_channel_one(
        &mut self,
        world: &mut impl CamGameWorld,
        singleton: Handle32<SingletonTag>,
    ) {
        let gate = world.blend_gate_one();
        let ceiling = float_global(world, CamGlobal::Ceiling);
        let inhibited = (gate as u8) != 0
            || (world.read_global(CamGlobal::InhibitA) as u8) != 0
            || (world.read_global(CamGlobal::InhibitB) as u8) != 0;
        if inhibited {
            world.write_global(CamGlobal::Accumulator1, 0);
            world.write_global(CamGlobal::Done1, 0);
            return;
        }
        let active = world.singleton_field(singleton, SingletonField::ActiveByte) as u8;
        if active != 0 {
            let mut x0 = ordered_add(
                f32::from_bits(self.blend),
                float_global(world, CamGlobal::Step1),
            );
            let cap = float_global(world, CamGlobal::Cap);
            if cap > x0 {
                x0 = cap;
            }
            if x0 < 0.0 {
                x0 = 0.0;
            } else if x0 > ceiling {
                x0 = ceiling;
            }
            self.blend = x0.to_bits();
            let accumulator = float_global(world, CamGlobal::Accumulator1);
            if accumulator == 0.0 {
                let n = world.read_global(CamGlobal::Count).wrapping_add(1);
                world.write_global(CamGlobal::Count, counter_step(n));
            }
            let mut y0 = ordered_add(
                float_global(world, CamGlobal::Accumulator1Step),
                accumulator,
            );
            world.write_global(CamGlobal::Done1, 1);
            if y0 > ceiling {
                y0 = 1.0;
            }
            world.write_global(CamGlobal::Accumulator1, y0.to_bits());
        } else if (world.read_global(CamGlobal::Fallback) as u8) == 0 {
            world.write_global(CamGlobal::Accumulator1, 0);
            world.write_global(CamGlobal::Done1, 0);
        }
    }

    /// Second blend channel: the same shape as the first, with its own step
    /// and accumulator, gated by the second gate answer, the inhibit bytes
    /// and the singleton's mode word reading exactly 1.
    fn blend_channel_two(
        &mut self,
        world: &mut impl CamGameWorld,
        singleton: Handle32<SingletonTag>,
    ) {
        let gate = world.blend_gate_two();
        let gated = (gate as u8) != 0
            || (world.read_global(CamGlobal::InhibitA) as u8) != 0
            || (world.read_global(CamGlobal::InhibitB) as u8) != 0
            || world.singleton_field(singleton, SingletonField::ModeWord) != 1;
        if gated {
            world.write_global(CamGlobal::Accumulator2, 0);
            world.write_global(CamGlobal::Done2, 0);
            return;
        }
        let mut x0 = ordered_add(
            f32::from_bits(self.blend),
            float_global(world, CamGlobal::Step2),
        );
        let cap = float_global(world, CamGlobal::Cap);
        if cap > x0 {
            x0 = cap;
        }
        let ceiling = float_global(world, CamGlobal::Ceiling);
        let blended = if x0 < 0.0 {
            0.0
        } else if x0 > ceiling {
            ceiling
        } else {
            x0
        };
        self.blend = blended.to_bits();
        let mut z0 = ordered_add(
            float_global(world, CamGlobal::Accumulator2Step),
            float_global(world, CamGlobal::Accumulator2),
        );
        world.write_global(CamGlobal::Done2, 1);
        if z0 > ceiling {
            z0 = 1.0;
        }
        world.write_global(CamGlobal::Accumulator2, z0.to_bits());
    }
}
#[cfg(test)]
mod counter_tests {
    use super::counter_step;

    /// Counts from zero up: the step is the remainder after division by three.
    #[test]
    fn counter_step_small_counts_are_n_mod_three() {
        for n in 0..=200_000_u32 {
            assert_eq!(counter_step(n), n % 3, "count {n}");
        }
    }

    /// Negative words, read as signed: the step is the signed remainder.
    #[test]
    fn counter_step_negative_words_are_signed_remainders() {
        for n in -200_000_i32..=0 {
            assert_eq!(counter_step(n as u32), (n % 3) as u32, "word {n}");
        }
        assert_eq!(counter_step(0x8000_0000), 0xFFFF_FFFE);
        assert_eq!(counter_step(0x7FFF_FFFF), 1);
    }
}
