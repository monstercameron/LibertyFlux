//! Voice activation: the live/active flags, the voice bitset and the slots.
//!
//! Lifted from the verified rewrites of the `r-s472` activation pair (no
//! class): refresh activations and notify every live voice, and tear a
//! voice down. The 32-bit object carries the live/active flag bytes at
//! `+0x3230`/`+0x3231`, the bitset address at `+0x28a0` and 799 eight-byte
//! voice slots from `+0xfa8` (only each slot's first word, the voice
//! object, is ever read). The counter, lock handle, gate and filter are
//! shared globals, owned here by [`VoiceLocks`]. Numbered callees and the
//! slots' probe/notify virtual hooks become [`ActivationWorld`] methods.
//!
//! [`BoundSlot`] is the one-routine bound check living in this module: a
//! flag byte and an id word tested against the current global id.

use lf_core::Handle32;

/// Live flag offset from the object start.
pub const LIVE_OFF: u32 = 0x3230;
/// Active flag offset from the object start.
pub const ACTIVE_OFF: u32 = 0x3231;
/// Bitset address offset from the object start.
pub const BITSET_OFF: u32 = 0x28a0;
/// First voice slot offset from the object start.
pub const SLOTS_OFF: u32 = 0xfa8;
/// Stride of one voice slot in bytes (only the first word is read).
pub const SLOT_STRIDE: u32 = 8;
/// Slots are numbered 1 below this bound: 799 slots.
pub const VOICES: u32 = 0x320;
/// How many voice slots one object holds.
pub const SLOT_COUNT: usize = 799;
/// Words of the voice bitset (slots 1..800 need bits to word 24).
pub const BITSET_WORDS: usize = 25;
/// Probe hook slot in a voice object's table.
pub const PROBE_SLOT: u32 = 0x14;
/// Notify hook slot in a voice object's table.
pub const NOTIFY_SLOT: u32 = 0x0c;

/// Identity of the lock behind the shared handle word.
#[derive(Debug)]
pub struct LockTag;

/// An opaque lock handle.
pub type LockHandle = Handle32<LockTag>;

/// Identity of a slotted voice object.
#[derive(Debug)]
pub struct SlotTag;

/// An opaque slotted voice: the object word of a voice slot.
pub type SlotHandle = Handle32<SlotTag>;

/// The voice bitset: one bit per slot number (bit `i` of word `i >> 5`).
pub type BitSet = [u32; BITSET_WORDS];

/// The activation globals, owned: lock handle, shared counter, gate and filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoiceLocks {
    /// The lock handle (the handle global; `None` is the null word).
    pub handle: Option<LockHandle>,
    /// The shared counter word.
    pub counter: u32,
    /// The gate flag (the gate global's low byte, nonzero is set).
    pub gate: bool,
    /// The filter flag (the filter global's low byte, nonzero is set).
    pub filter: bool,
}

/// One voice's activation state: flags, bitset and slots.
pub struct VoiceActivation {
    /// The live flag (`+0x3230`, nonzero is set).
    pub live: bool,
    /// The active flag (`+0x3231`, nonzero is set).
    pub active: bool,
    /// The voice bitset (`None` is the null address: the update faults).
    pub bits: Option<BitSet>,
    /// The 799 voice slots (`None` is the null object: hooks fault).
    pub slots: [Option<SlotHandle>; SLOT_COUNT],
}

impl VoiceActivation {
    /// The lock/unlock pair around a counter step, as both routines run it.
    ///
    /// The 32-bit form reads the handle global twice (once per call);
    /// over owned state the two reads cannot disagree, so one read
    /// serves both (see the registry).
    fn locked(shared: &mut VoiceLocks, world: &mut impl ActivationWorld, up: bool) {
        let handle = shared.handle;
        world.lock(handle);
        shared.counter = if up {
            shared.counter.wrapping_add(1)
        } else {
            shared.counter.wrapping_sub(1)
        };
        world.unlock(handle);
    }

    /// Refreshes this voice's activation, then notifies every live voice.
    ///
    /// A clear live flag ends the work, as does a clear gate with a
    /// settled active flag. Otherwise an active voice with the gate
    /// clear is deactivated (reset, locked decrement) and the work ends,
    /// while an inactive voice with the gate set is activated (locked
    /// increment) and falls through. The main loop scans slots 1..800:
    /// for each set bit, the filter flag selects a probe first (a zero
    /// low byte skips the slot), then the notify hook runs with `arg`.
    ///
    /// # Panics
    ///
    /// When the bitset is null or a notified slot holds the null object
    /// (the original faults through null either way).
    pub fn update(&mut self, arg: u32, shared: &mut VoiceLocks, world: &mut impl ActivationWorld) {
        if !self.live {
            return;
        }
        if self.active {
            if !shared.gate {
                world.reset();
                Self::locked(shared, world, false);
                self.active = false;
                return;
            }
        } else {
            if !shared.gate {
                return;
            }
            Self::locked(shared, world, true);
            self.active = true;
        }
        let bits = self
            .bits
            .as_ref()
            .unwrap_or_else(|| panic!("null voice bitset: the original faults through null"));
        let mut mask = 2u32;
        for (k, slot) in self.slots.iter().enumerate() {
            let i = u32::try_from(k).expect("799 slot indexes fit u32") + 1;
            let w = bits[(i >> 5) as usize];
            if w & mask != 0 {
                let obj = slot.unwrap_or_else(|| {
                    panic!("null voice object in live slot {i}: the original faults through null")
                });
                let mut run = true;
                if shared.filter {
                    run = world.probe(obj);
                }
                if run {
                    world.notify(obj, arg);
                }
            }
            mask = mask.rotate_left(1);
        }
    }

    /// Tears a voice down: locked decrement, free the bitset, clear flags.
    ///
    /// A clear live flag ends the work. Otherwise an active voice is
    /// deactivated first (reset, locked decrement), then the bitset is
    /// handed to the world for freeing and both flags clear.
    pub fn teardown(&mut self, shared: &mut VoiceLocks, world: &mut impl ActivationWorld) {
        if !self.live {
            return;
        }
        if self.active {
            world.reset();
            Self::locked(shared, world, false);
        }
        self.active = false;
        // The free call runs even for the null bitset (with the null
        // word); the lift hands zero words then (see the registry).
        world.free_bits(self.bits.take().unwrap_or([0; BITSET_WORDS]));
        self.live = false;
    }
}

/// The activation routines' collaborators: the numbered callees plus the
/// slots' probe/notify virtual hooks.
///
/// The reset callee also receives the voice's own address; it carries no
/// meaning in the lift (the voice is `self`) and is dropped, with the
/// proof pinning the planted address per case.
pub trait ActivationWorld {
    /// Resets the voice being (de)activated: the reset callee.
    fn reset(&mut self);
    /// Takes the shared lock: the lock callee.
    fn lock(&mut self, handle: Option<LockHandle>);
    /// Releases the shared lock: the unlock callee.
    fn unlock(&mut self, handle: Option<LockHandle>);
    /// Frees a voice bitset: the free callee, taking the owned words.
    fn free_bits(&mut self, bits: BitSet);
    /// Probes a slotted voice: its virtual probe hook. The 32-bit hook
    /// answers a word of which only the low byte decides; the lift takes
    /// the decided bool (the proof scripts words whose full value and
    /// low byte disagree).
    fn probe(&mut self, slot: SlotHandle) -> bool;
    /// Notifies a slotted voice: its virtual notify hook.
    fn notify(&mut self, slot: SlotHandle, arg: u32);
}

/// A bound voice slot: the flag byte and id word the bound check reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundSlot {
    /// The flag byte (slot `+0x11c`).
    pub flag: u8,
    /// The bound id (slot `+0x114`).
    pub id: u32,
}

impl BoundSlot {
    /// Whether the slot is actively bound: the flag is nonzero, the id
    /// is nonzero, and it differs from the `current` global id.
    #[must_use]
    pub fn is_active(&self, current: u32) -> bool {
        self.flag != 0 && self.id != 0 && self.id != current
    }
}
