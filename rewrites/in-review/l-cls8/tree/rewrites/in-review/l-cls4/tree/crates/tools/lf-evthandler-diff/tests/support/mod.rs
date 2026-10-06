//! Shared differential-test support: generator, edge values, object blobs.
//!
//! Included by each diff test target (`#[path]`), so every binary gets
//! its own copy. 32-bit only: addresses are real.

// Helpers are shared across binaries; each binary uses its own subset.
#![allow(dead_code)]

/// Small deterministic generator (splitmix64).
pub struct Rng(pub u64);

impl Rng {
    /// Next 64 bits.
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Next word.
    pub fn u32(&mut self) -> u32 {
        (self.next() >> 32) as u32
    }
}

/// Edge words: zero, one, small values, the sign boundary, all ones.
pub const U32_EDGE: [u32; 10] = [
    0,
    1,
    2,
    3,
    0x7FFF_FFFF,
    0x8000_0000,
    0xFFFF_FFFE,
    0xFFFF_FFFF,
    0x5E,
    0xDEAD_BEEF,
];

/// Address of a referent as the rewrites take it (32-bit target only).
pub fn addr<T>(r: &T) -> u32 {
    (r as *const T).addr() as u32
}

/// Handler blob words: table, owner, an unread pad word, pending task.
pub const H_VTABLE: usize = 0;
/// Owner word of the handler blob.
pub const H_OWNER: usize = 1;
/// Unread pad word of the handler blob (filled random, must survive).
pub const H_PAD: usize = 2;
/// Pending-task word of the handler blob.
pub const H_PENDING: usize = 3;

/// Event blob word holding the table pointer.
pub const E_VTABLE: usize = 0;
/// Event blob word holding the kind (byte offset 0x10).
pub const E_KIND: usize = 4;
/// Event blob word holding the payload (byte offset 0x18).
pub const E_PAYLOAD: usize = 6;

/// Zeroed 16-byte handler blob (word-addressed, so aligned).
pub fn handler_blob() -> Box<[u32; 4]> {
    Box::new([0u32; 4])
}

/// Zeroed 32-byte event blob (word-addressed, so aligned).
pub fn event_blob() -> Box<[u32; 8]> {
    Box::new([0u32; 8])
}

/// Zeroed fake virtual table with one stub planted at a word slot.
pub fn fake_table(words: usize, slot_word: usize, stub: u32) -> Box<[u32]> {
    let mut t = vec![0u32; words].into_boxed_slice();
    t[slot_word] = stub;
    t
}
