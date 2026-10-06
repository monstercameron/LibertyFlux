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

/// Fist blob word holding the held slot (byte offset 0x18).
pub const F_HELD: usize = 6;
/// Fist blob word holding the member word (byte offset 0x1c).
pub const F_LINK: usize = 7;

/// Hit blob word holding the kind (byte offset 0x14).
pub const H_KIND: usize = 5;

/// Duck blob word holding the table pointer.
pub const D_VTABLE: usize = 0;
/// Duck blob word holding the marks (byte offset 0x0c).
pub const D_MARKS: usize = 3;
/// Duck blob word holding the start tick (byte offset 0x14).
pub const D_START: usize = 5;
/// Duck blob word holding the span (byte offset 0x18).
pub const D_SPAN: usize = 6;
/// Duck blob word holding the level in its low half (byte offset 0x1c).
pub const D_LEVEL: usize = 7;
/// Duck blob word holding the done/flagged bytes (byte offset 0x28).
pub const D_FLAGW: usize = 10;
/// Duck blob word holding the tag byte (byte offset 0x38).
pub const D_TAGW: usize = 14;
/// Byte offset of the done flag in the duck blob.
pub const D_DONE_BYTE: usize = 0x29;
/// Byte offset of the event flag in the duck blob.
pub const D_FLAGGED_BYTE: usize = 0x2a;
/// Byte offset of the tag byte in the duck blob.
pub const D_TAG_BYTE: usize = 0x38;

/// Query blob word holding the table pointer.
pub const Q_VTABLE: usize = 0;
/// Query blob word holding the state (byte offset 0x10).
pub const Q_STATE: usize = 4;

/// Flee blob word holding the table pointer.
pub const FL_VTABLE: usize = 0;
/// Flee blob word holding the subtask (byte offset 0x08).
pub const FL_SUB: usize = 2;
/// Flee blob word holding the marks (byte offset 0x0c).
pub const FL_MARKS: usize = 3;
/// Flee blob word holding the kind (byte offset 0x20).
pub const FL_KIND: usize = 8;
/// Flee blob words holding the position (byte offset 0x30).
pub const FL_POS: usize = 12;
/// Byte offset of the gate flag in the flee blob.
pub const FL_FLAG_BYTE: usize = 0x46;
/// Flee blob word holding the mode (byte offset 0x48).
pub const FL_MODE: usize = 18;
/// Byte offset of the state byte in the flee blob.
pub const FL_STATE_BYTE: usize = 0x70;

/// Goto blob word holding the subtask (byte offset 0x08).
pub const G_SUB: usize = 2;
/// Goto blob word holding the kind (byte offset 0x20).
pub const G_KIND: usize = 8;
/// Goto blob words holding the position (byte offset 0x30).
pub const G_POS: usize = 12;
/// Byte offset of the gate flag in the goto blob.
pub const G_FLAG_BYTE: usize = 0x46;
/// Goto blob word holding the mode (byte offset 0x48).
pub const G_MODE: usize = 18;
/// Goto blob word holding the wait (byte offset 0x70).
pub const G_WAIT: usize = 28;
/// Goto blob word holding the stamp (byte offset 0x74).
pub const G_STAMP: usize = 29;
/// Goto blob word holding the wait copy (byte offset 0x78).
pub const G_WAITCP: usize = 30;
/// Byte offset of the armed byte in the goto blob.
pub const G_ARMED_BYTE: usize = 0x7c;
/// Byte offset of the restamp byte in the goto blob.
pub const G_RESTAMP_BYTE: usize = 0x7d;
/// Goto blob word holding the speed (byte offset 0x80).
pub const G_SPEED: usize = 32;

/// Zeroed 32-byte fist blob (word-addressed, so aligned).
pub fn fist_blob() -> Box<[u32; 8]> {
    Box::new([0u32; 8])
}

/// Zeroed 32-byte hit blob (word-addressed, so aligned).
pub fn hit_blob() -> Box<[u32; 8]> {
    Box::new([0u32; 8])
}

/// Zeroed 72-byte duck blob (word-addressed, so aligned).
pub fn duck_blob() -> Box<[u32; 18]> {
    Box::new([0u32; 18])
}

/// Zeroed 32-byte query blob (word-addressed, so aligned).
pub fn query_blob() -> Box<[u32; 8]> {
    Box::new([0u32; 8])
}

/// Zeroed 16-byte ped blob (word-addressed, so aligned).
pub fn ped_blob() -> Box<[u32; 4]> {
    Box::new([0u32; 4])
}

/// Zeroed 120-byte flee blob (word-addressed, so aligned).
pub fn flee_blob() -> Box<[u32; 30]> {
    Box::new([0u32; 30])
}

/// Zeroed 136-byte goto blob (word-addressed, so aligned).
pub fn goto_blob() -> Box<[u32; 34]> {
    Box::new([0u32; 34])
}

/// Big ped blob word holding the probe object (byte offset 0x224).
pub const P_PROBEW: usize = 137;

/// Entity blob word holding the kind word (byte offset 0x28).
pub const E_KINDW: usize = 10;

/// Built-reaction blob word holding the marks (byte offset 0x60).
pub const B_MARKSW: usize = 24;

/// Zeroed 560-byte ped blob, wide enough for the probe word.
pub fn big_ped_blob() -> Box<[u32; 140]> {
    Box::new([0u32; 140])
}

/// Zeroed 48-byte entity blob, wide enough for the kind word.
pub fn entity_blob() -> Box<[u32; 12]> {
    Box::new([0u32; 12])
}

/// Zeroed 104-byte built-reaction blob, wide enough for the marks.
pub fn built_blob() -> Box<[u32; 26]> {
    Box::new([0u32; 26])
}

/// Zeroed 8-byte subtask blob (table pointer first).
pub fn sub_blob() -> Box<[u32; 2]> {
    Box::new([0u32; 2])
}

/// Byte offset of the ped type half-word in the event ped blob.
pub const P_TYPE_OFF: usize = 0x2E;
/// Byte offset of the ped flag byte in the event ped blob.
pub const P_FLAGS_BYTE: usize = 0x26C;
/// Byte offset of the ped auxiliary flag byte in the event ped blob.
pub const P_AUX_BYTE: usize = 0xF4;
/// Event ped blob word holding the fallback target (byte offset 0xb30).
pub const P_TARGETW: usize = 716;
/// Byte offset of the seed object past the ped.
pub const P_SEED_OFF: u32 = 0x570;
/// Built task-B blob word holding the marks (byte offset 0x6c).
pub const B_MARKSW_B: usize = 27;
/// Byte offset of the status byte in a table entry blob.
pub const E_STATUS_BYTE: usize = 0xEE;
/// Table entries below the live window (the most negative type id).
pub const TABLE_NEG: usize = 32768;
/// Table entries in the planted ped-type table.
pub const TABLE_LEN: usize = 98304;

/// Zeroed event ped blob, wide enough for the target word.
pub fn event_ped_blob() -> Box<[u32; 720]> {
    Box::new([0u32; 720])
}

/// Zeroed 104-byte built task-B blob, wide enough for its marks.
pub fn built_b_blob() -> Box<[u32; 28]> {
    Box::new([0u32; 28])
}

/// Zeroed table entry blob, wide enough for the status byte.
pub fn entry_blob() -> Box<[u32; 60]> {
    Box::new([0u32; 60])
}

/// Reads one half-word of a word blob (little-endian).
pub fn blob_u16(blob: &[u32], byte: usize) -> u16 {
    let words: &[u8] =
        unsafe { core::slice::from_raw_parts(blob.as_ptr().cast::<u8>(), blob.len() * 4) };
    u16::from_le_bytes([words[byte], words[byte + 1]])
}

/// Writes one half-word of a word blob (little-endian).
pub fn set_blob_u16(blob: &mut [u32], byte: usize, value: u16) {
    let words: &mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(blob.as_mut_ptr().cast::<u8>(), blob.len() * 4) };
    let bytes = value.to_le_bytes();
    words[byte] = bytes[0];
    words[byte + 1] = bytes[1];
}

/// Reads one byte of a word blob.
pub fn blob_byte(blob: &[u32], byte: usize) -> u8 {
    let words: &[u8] =
        unsafe { core::slice::from_raw_parts(blob.as_ptr().cast::<u8>(), blob.len() * 4) };
    words[byte]
}

/// Writes one byte of a word blob.
pub fn set_blob_byte(blob: &mut [u32], byte: usize, value: u8) {
    let words: &mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(blob.as_mut_ptr().cast::<u8>(), blob.len() * 4) };
    words[byte] = value;
}

/// Zeroed fake virtual table with one stub planted at a word slot.
pub fn fake_table(words: usize, slot_word: usize, stub: u32) -> Box<[u32]> {
    let mut t = vec![0u32; words].into_boxed_slice();
    t[slot_word] = stub;
    t
}

/// Zeroed fake virtual table with two stubs planted at word slots.
pub fn fake_table2(
    words: usize,
    slot_a: usize,
    stub_a: u32,
    slot_b: usize,
    stub_b: u32,
) -> Box<[u32]> {
    let mut t = vec![0u32; words].into_boxed_slice();
    t[slot_a] = stub_a;
    t[slot_b] = stub_b;
    t
}
