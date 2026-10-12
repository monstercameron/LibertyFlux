//! Small pool resets: handle state, slot, row pairs and slot pairs.
//!
//! Lifted from five verified routines (no class) that each wipe one small
//! region: the handle-state reset, the slot reset, the 16-row pair wipe,
//! and the two word-pair setters (one routine, proved twice). Each type
//! owns its region as bytes with typed access to the words that matter.

/// Size of the handle-state region: words up to +0x390 plus the word.
pub const HANDLE_SIZE: usize = 0x394;
/// Flag bit the handle-state reset clears.
pub const HANDLE_BIT: u8 = 0x01;
/// Size of the small-slot region: words up to +0x6c plus the word.
pub const SLOT_SIZE: usize = 0x70;
/// Flag bit the slot reset clears.
pub const SLOT_BIT: u8 = 0x02;
/// Offset of the shared flag byte in both regions.
pub const FLAG_OFF: usize = 0x5b;
/// Rows wiped by the pair wipe.
pub const ROW_PAIRS: usize = 16;
/// Row stride of the pair wipe.
pub const ROW_STRIDE: usize = 0x30;
/// Size of the pair-wipe region: the end cursor.
pub const ROW_SIZE: usize = 0x384;
/// Size of the slot-pair region: words up to +0xA14 plus the word.
pub const PAIR_SIZE: usize = 0xA18;

/// A pool-handle wrapper's cached state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandleState {
    /// The region.
    img: [u8; HANDLE_SIZE],
}

impl HandleState {
    /// A state over its region bytes.
    #[must_use]
    pub fn from_bytes(img: [u8; HANDLE_SIZE]) -> Self {
        Self { img }
    }

    /// The region bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.img
    }

    /// Resets the cached state: clears flag bit 0 at +0x5b and zeroes the
    /// words at +0x40, +0x44, +0x48 and +0x390. (The 32-bit form answers
    /// the object address, which carries no meaning past the call.)
    pub fn reset(&mut self) {
        self.img[FLAG_OFF] &= !HANDLE_BIT;
        for off in [0x40, 0x44, 0x48, 0x390] {
            self.img[off..off + 4].copy_from_slice(&0u32.to_le_bytes());
        }
    }
}

/// A small pool slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmallSlot {
    /// The region.
    img: [u8; SLOT_SIZE],
}

impl SmallSlot {
    /// A slot over its region bytes.
    #[must_use]
    pub fn from_bytes(img: [u8; SLOT_SIZE]) -> Self {
        Self { img }
    }

    /// The region bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.img
    }

    /// Resets the slot: clears flag bit 1 at +0x5b and zeroes the words
    /// at +0x4, +0x18 and +0x6c. (The original leaves the return register
    /// untouched; the rewrite answers 0.)
    pub fn reset(&mut self) {
        self.img[FLAG_OFF] &= !SLOT_BIT;
        for off in [0x04, 0x18, 0x6c] {
            self.img[off..off + 4].copy_from_slice(&0u32.to_le_bytes());
        }
    }
}

/// Sixteen rows of two pointer slots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowPairs {
    /// The region.
    img: [u8; ROW_SIZE],
}

impl RowPairs {
    /// Rows over their region bytes.
    #[must_use]
    pub fn from_bytes(img: [u8; ROW_SIZE]) -> Self {
        Self { img }
    }

    /// The region bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.img
    }

    /// Zeroes both slots of all sixteen rows (at +0x80 up to +0x380),
    /// answering the end offset. (The 32-bit form answers the end cursor
    /// as an address.)
    pub fn zero_all(&mut self) -> usize {
        let mut cur = 0x84;
        for _ in 0..ROW_PAIRS {
            self.img[cur - 4..cur].copy_from_slice(&0u32.to_le_bytes());
            self.img[cur..cur + 4].copy_from_slice(&0u32.to_le_bytes());
            cur += ROW_STRIDE;
        }
        cur
    }
}

/// Which slot pair a word-pair setter writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairSlot {
    /// The pair at +0xA08/+0xA0C.
    Low,
    /// The pair at +0xA10/+0xA14.
    High,
}

impl PairSlot {
    /// Offset of the pair's first word.
    #[must_use]
    pub const fn offset(self) -> usize {
        match self {
            PairSlot::Low => 0xA08,
            PairSlot::High => 0xA10,
        }
    }
}

/// A pool object fragment carrying two word pairs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotPair {
    /// The region.
    img: [u8; PAIR_SIZE],
}

impl SlotPair {
    /// A fragment over its region bytes.
    #[must_use]
    pub fn from_bytes(img: [u8; PAIR_SIZE]) -> Self {
        Self { img }
    }

    /// The region bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.img
    }

    /// Copies the source pair into the slots, answering the second word
    /// (as the original leaves it in its return register).
    pub fn set_pair(&mut self, slot: PairSlot, w0: u32, w1: u32) -> u32 {
        let off = slot.offset();
        self.img[off..off + 4].copy_from_slice(&w0.to_le_bytes());
        self.img[off + 4..off + 8].copy_from_slice(&w1.to_le_bytes());
        w1
    }
}
