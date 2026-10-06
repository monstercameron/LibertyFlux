//! Single-routine audio-slot neighbours: one routine each, sharing this module.
//!
//! The tracker cell ([`TrackerCell`]) adds into a slot word and
//! dispatches; the slot header ([`SlotHead`]) is the slot object's
//! constructor; the owned slot ([`OwnedSlot`]) releases through a gate
//! over subsystem words; the liveness probe ([`LivenessProbe`]) gates on
//! float tags before handing to the mixer.

/// Tracker slot word offset.
pub const TRACKER_OFF: u32 = 0xB88;
/// Owned-slot word offset.
pub const OWNED_OFF: u32 = 0x14;
/// Parameter word offset inside the owned slot's object.
pub const OWNED_PARAM_OFF: u32 = 0xA4;
/// Slot header stamp.
pub const HEAD_STAMP: u16 = 0x100;
/// Slot header flag bits kept from the old byte.
pub const HEAD_FLAG_KEEP: u8 = 0xFA;
/// Slot header mode bit set by the constructor.
pub const HEAD_FLAG_MODE: u8 = 2;
/// Liveness tag byte offset.
pub const LIVE_TAG_OFF: u32 = 0x00;
/// Liveness voice-view offset.
pub const LIVE_VIEW_OFF: u32 = 0x20;
/// Liveness zero-tag word offset.
pub const LIVE_ZERO_OFF: u32 = 0x64;
/// Liveness positive-tag word offset.
pub const LIVE_POS_OFF: u32 = 0x68;
/// Liveness failing-path constant top half.
pub const LIVE_FAIL_TOP: u32 = 0xA5A5_0000;

/// Runs the dispatched slot store: the tracker's callee.
pub trait Dispatch {
    /// Stores `sum` for the tracker, answering as the 32-bit callee.
    fn dispatch(&mut self, sum: u32) -> u32;
}

impl<F: FnMut(u32) -> u32> Dispatch for F {
    fn dispatch(&mut self, sum: u32) -> u32 {
        self(sum)
    }
}

/// One tracker slot word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackerCell(pub u32);

impl TrackerCell {
    /// Adds the slot word into `a` (wrapping) and dispatches the sum,
    /// returning the dispatch answer.
    pub fn add_and_dispatch(&self, a: u32, dispatch: &mut impl Dispatch) -> u32 {
        dispatch.dispatch(a.wrapping_add(self.0))
    }
}

/// A fresh slot header: the slot object's constructor.
///
/// Every word is zero and the header stamp is [`HEAD_STAMP`]; only the
/// flag byte keeps information, mapping old `f` to
/// `(f & HEAD_FLAG_KEEP) | HEAD_FLAG_MODE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotHead {
    /// Flag byte after the constructor's mapping.
    pub flag: u8,
}

impl SlotHead {
    /// Builds a fresh header from the old flag byte `flag`.
    #[must_use]
    pub const fn fresh(flag: u8) -> Self {
        Self {
            flag: (flag & HEAD_FLAG_KEEP) | HEAD_FLAG_MODE,
        }
    }
}

/// The subsystem words behind the release gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateState {
    /// First gate word: live unless exactly 1.
    pub g1: u32,
    /// Second gate word: live when equal to [`GateState::g3`].
    pub g2: u32,
    /// Third gate word.
    pub g3: u32,
    /// Fourth gate word: live unless exactly `0x12`.
    pub g4: u32,
}

impl GateState {
    /// Whether the bank lookup runs before the release.
    #[must_use]
    pub const fn live(&self) -> bool {
        self.g1 != 1 && self.g2 == self.g3 && self.g4 != 0x12
    }
}

/// A slot reference where the 32-bit code passes the slot's address:
/// opaque, never dereferenced here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotRef(pub u32);

/// The gated release's reads and callees.
pub trait GatedRelease {
    /// Reads the parameter word of `slot`'s object (at `+0xA4`).
    fn slot_param(&mut self, slot: SlotRef) -> u32;
    /// The bank lookup on the parameter word.
    fn notify(&mut self, param: u32);
    /// The release call on the slot.
    fn release(&mut self, slot: SlotRef);
}

/// An owned slot word: 0 means absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnedSlot(pub u32);

impl OwnedSlot {
    /// Releases the owned slot, looking the bank up first when live.
    ///
    /// An absent slot does nothing. Otherwise, when the gate words read
    /// live, the slot's parameter word runs through the bank lookup;
    /// then the release call runs on the slot. (The 32-bit code re-reads
    /// the slot word between the two calls; the lookup is assumed not to
    /// retarget it, and the proof pins that the release receives the
    /// planted slot on every case.)
    pub fn release_gated(&self, gates: &GateState, ops: &mut impl GatedRelease) {
        if self.0 == 0 {
            return;
        }
        let slot = SlotRef(self.0);
        if gates.live() {
            let param = ops.slot_param(slot);
            ops.notify(param);
        }
        ops.release(slot);
    }
}

/// The liveness probe's mixer call.
pub trait Mixer {
    /// Mixes `first` with the object's voice view, answering as the
    /// 32-bit mixer. The view is fixed (the object at `+0x20`), so only
    /// the forwarded word crosses the trait; the proof pins the view
    /// address per case.
    fn mix(&mut self, first: u32) -> u32;
}

impl<F: FnMut(u32) -> u32> Mixer for F {
    fn mix(&mut self, first: u32) -> u32 {
        self(first)
    }
}

/// The liveness probe: tag byte plus the two float tag words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LivenessProbe {
    /// Tag byte (`+0x00`): zero fails the gate.
    pub tag: u8,
    /// Zero-tag word (`+0x64`): must read as positive or negative zero.
    pub zero: f32,
    /// Positive-tag word (`+0x68`): must read above zero.
    pub pos: f32,
}

impl LivenessProbe {
    /// Gates on the tags, then hands to the mixer.
    ///
    /// Passes only when the tag byte is nonzero, `zero` compares equal
    /// to zero and `pos` compares above zero (either float check fails
    /// on a not-a-number). On the passing path the mixer runs on `first`
    /// and its answer is returned. On any failing path answers the
    /// failing constant: top half [`LIVE_FAIL_TOP`], second byte the
    /// flag byte the zero check leaves behind (`0x47` for NaN, `0x42`
    /// for zero, `0x03` for negative, `0x02` otherwise), low byte one
    /// when both float checks passed and zero otherwise.
    pub fn gate(&self, first: u32, mixer: &mut impl Mixer) -> u32 {
        let latch = u8::from(self.zero == 0.0 && self.pos > 0.0);
        if self.tag == 0 || latch == 0 {
            return LIVE_FAIL_TOP | (Self::flag_byte(self.zero) << 8) | u32::from(latch);
        }
        mixer.mix(first)
    }

    /// The flag byte the zero check leaves behind.
    fn flag_byte(zero: f32) -> u32 {
        // NaN, zero (either sign), negative, otherwise: the ladder from
        // the verified rewrite, in order.
        if zero.is_nan() {
            0x47
        } else if zero == 0.0 {
            0x42
        } else if zero < 0.0 {
            0x03
        } else {
            0x02
        }
    }
}
