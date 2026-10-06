//! Packed point-plus-key skip calls: one routine in three flag instances.
//!
//! Lifted from the three verified `script_vm_pack_words_*` rewrites. Each
//! packs three words into a point triple and forwards it with a key word
//! and three flag words; the instances differ only in the flags, so all
//! three are proved against the one generic [`PointSkip::emit`].

/// The three flag words a skip instance forwards after its key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkipFlags(pub u32, pub u32, pub u32);

impl SkipFlags {
    /// Flags of the all-zero instance: no flag set.
    pub const NONE: Self = Self(0, 0, 0);
    /// Flags of the set-flag instance: the first flag word set.
    pub const FIRST_SET: Self = Self(1, 0, 0);
    /// Flags of the trailing-flag instance: the last flag word set.
    pub const TRAIL_SET: Self = Self(0, 0, 1);
}

/// Receives a packed skip call: the engine routine behind the thunks.
pub trait SkipSink {
    /// Handles the packed `point` with its `key` and `flags`.
    fn skip(&mut self, point: [u32; 3], key: u32, flags: SkipFlags);
}

impl<F: FnMut([u32; 3], u32, SkipFlags)> SkipSink for F {
    fn skip(&mut self, point: [u32; 3], key: u32, flags: SkipFlags) {
        self(point, key, flags);
    }
}

/// The pack-and-skip protocol, shared by the three flag instances.
///
/// The 32-bit routines keep no state: they pack their three words on the
/// frame and forward them, so the lift is a stateless translator from
/// script words to a [`SkipSink`] call.
#[derive(Debug, Default, Clone, Copy)]
pub struct PointSkip;

impl PointSkip {
    /// Packs (`a0`, `a1`, `a2`) into a point and forwards it with `key`
    /// and the instance's `flags`.
    pub fn emit(
        &self,
        sink: &mut impl SkipSink,
        a0: u32,
        a1: u32,
        a2: u32,
        key: u32,
        flags: SkipFlags,
    ) {
        sink.skip([a0, a1, a2], key, flags);
    }
}
