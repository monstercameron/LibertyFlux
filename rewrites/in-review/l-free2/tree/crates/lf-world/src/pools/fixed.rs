//! Fixed-address pool tables: the 20-byte entry table and the 44-byte table.
//!
//! Lifted from the five verified routines over the game's two fixed tables
//! (no class): the entry table's initialiser and find-and-mark, and the
//! revocation table's reset, clear and revoke. Both tables live at fixed
//! addresses in the original; the lift owns the entries as ordinary Rust
//! data and the base addresses stay at the boundary (pinned per case).

/// Entries in the 20-byte table.
pub const ENTRY_COUNT: usize = 0x400;
/// Bytes per entry of the 20-byte table.
pub const ENTRY_LEN: usize = 20;
/// Miss answer of the find-and-mark: `0x3FF * 5`.
pub const FIND_MISS: u32 = 0x13fb;
/// Hit answer per index: the index times 5.
pub const FIND_SCALE: u32 = 5;
/// Bias of the initialiser's answer: the 32-bit form computes the end from
/// a pointer 8 past the table start, so it answers end + 8.
pub const INIT_END_BIAS: usize = 8;
/// Tag the initialiser stamps into every entry.
pub const INIT_TAG: u16 = 0xffff;

/// One 20-byte entry: the pointer, tag, key, sum and flag the routines
/// read and write, plus the bytes both routines leave untouched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Pointer word at +0: null means the find-and-mark skips the entry.
    pub ptr: u32,
    /// Tag half at +4.
    pub tag: u16,
    /// Untouched half at +6.
    pub gap: u16,
    /// Key word at +8.
    pub key: u32,
    /// Sum word at +0xC.
    pub sum: u32,
    /// Flag byte at +0x10.
    pub flag: u8,
    /// Untouched bytes at +0x11..+0x14.
    pub pad: [u8; 3],
    /// Word at +16.
    pub word16: u16,
    /// Untouched bytes at +18..+19.
    pub tail: [u8; 2],
}

impl Entry {
    /// Reads one entry from 20 bytes.
    #[must_use]
    pub fn from_bytes(b: &[u8]) -> Self {
        Self {
            ptr: u32::from_le_bytes(b[0..4].try_into().unwrap()),
            tag: u16::from_le_bytes(b[4..6].try_into().unwrap()),
            gap: u16::from_le_bytes(b[6..8].try_into().unwrap()),
            key: u32::from_le_bytes(b[8..12].try_into().unwrap()),
            sum: u32::from_le_bytes(b[12..16].try_into().unwrap()),
            flag: b[16],
            pad: b[17..20].try_into().unwrap(),
            word16: 0,
            tail: [0, 0],
        }
    }
}
