//! Small resettable streaming objects: slots, lane tables, id arrays.
//!
//! Lifted from the verified rewrites. Three one-routine shapes share
//! this module: the slot record reset in place, the 11-lane table
//! reset in place, and the dense id array searched from a start
//! index. Returned object pointers narrow away; the proofs check the
//! rewrites answer the planted objects.

/// Byte length of one resettable slot.
pub const SLOT_LEN: usize = 0x88;
/// Flag bits the slot reset keeps.
const KEEP_MASK: u8 = 0xE0;
/// First and last word offsets the slot reset zeroes.
const FIRST_WORD: usize = 0x08;
const LAST_WORD: usize = 0x84;
/// Lane count of the lane table.
pub const LANES: usize = 11;
/// Byte length of the lane table: two rows, a tail word, a tag byte.
pub const LANE_TABLE_LEN: usize = 0x5d;
/// Fresh link word of the lane table's second row.
const FRESH_LINK: u32 = 0xffff_ffff;

/// One resettable streaming slot: 136 owned bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResetSlot {
    /// Slot bytes in address order.
    pub bytes: [u8; SLOT_LEN],
}

impl ResetSlot {
    /// Resets this slot in place.
    ///
    /// Restates `stream_slot_reset`: keeps only the top three bits of
    /// the flag byte at `+0`, clears the tag byte at `+1`, and zeroes
    /// every word from `+0x08` through `+0x84` (bytes `+2..+8` are
    /// left alone). The returned slot pointer narrows away.
    pub fn reset(&mut self) {
        self.bytes[0] &= KEEP_MASK;
        self.bytes[1] = 0;
        let mut off = FIRST_WORD;
        while off <= LAST_WORD {
            self.bytes[off..off + 4].copy_from_slice(&0u32.to_le_bytes());
            off += 4;
        }
    }
}

/// One 11-lane streaming table: two rows, a tail word, a tag byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LaneTable {
    /// First row: cleared by the reset.
    pub row0: [u32; LANES],
    /// Second row: all-ones after the reset.
    pub row1: [u32; LANES],
    /// Tail word at `+0x58`: cleared by the reset.
    pub tail: u32,
    /// Tag byte at `+0x5c`: cleared by the reset.
    pub tag: u8,
}

impl LaneTable {
    /// Decodes a table from its bytes, little-endian.
    #[must_use]
    pub fn from_bytes(bytes: [u8; LANE_TABLE_LEN]) -> Self {
        let mut row0 = [0u32; LANES];
        let mut row1 = [0u32; LANES];
        let mut i = 0;
        while i < LANES {
            let a = i * 4;
            row0[i] = u32::from_le_bytes([bytes[a], bytes[a + 1], bytes[a + 2], bytes[a + 3]]);
            let b = 0x2c + i * 4;
            row1[i] = u32::from_le_bytes([bytes[b], bytes[b + 1], bytes[b + 2], bytes[b + 3]]);
            i += 1;
        }
        Self {
            row0,
            row1,
            tail: u32::from_le_bytes([bytes[0x58], bytes[0x59], bytes[0x5a], bytes[0x5b]]),
            tag: bytes[0x5c],
        }
    }

    /// Encodes this table as its bytes, little-endian.
    #[must_use]
    pub fn to_bytes(self) -> [u8; LANE_TABLE_LEN] {
        let mut out = [0u8; LANE_TABLE_LEN];
        let mut i = 0;
        while i < LANES {
            out[i * 4..i * 4 + 4].copy_from_slice(&self.row0[i].to_le_bytes());
            out[0x2c + i * 4..0x2c + i * 4 + 4].copy_from_slice(&self.row1[i].to_le_bytes());
            i += 1;
        }
        out[0x58..0x5c].copy_from_slice(&self.tail.to_le_bytes());
        out[0x5c] = self.tag;
        out
    }

    /// Resets this table in place.
    ///
    /// Restates `stream_table_reset`: clears the first row, writes
    /// all-ones to the second row, and clears the tail word and the
    /// tag byte. The returned table pointer narrows away.
    pub fn reset(&mut self) {
        self.row0 = [0; LANES];
        self.row1 = [FRESH_LINK; LANES];
        self.tail = 0;
        self.tag = 0;
    }
}

/// A dense id array with its length word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdArray {
    /// Ids in scan order.
    pub ids: Vec<u32>,
    /// Length word, read signed by the scan.
    pub len: i32,
}

impl IdArray {
    /// Finds the first slot at or after `start` holding `want`.
    ///
    /// Restates `stream_find_index`: scans `start..len` with both
    /// bounds compared signed, answering the matching index or -1.
    /// The key pointer narrows to the wanted value; -1 narrows to
    /// `None`.
    ///
    /// # Panics
    ///
    /// When the scan would read a negative index or past the owned
    /// ids; the original reads on with wrapped addresses.
    #[must_use]
    // The casts are the behaviour: the original compares the start
    // signed and answers the index word. Both panic arms run first, so
    // the usize/u32 casts only see non-negative values.
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    pub fn find_from(&self, want: u32, start: u32) -> Option<u32> {
        let mut i = start as i32;
        while i < self.len {
            assert!(i >= 0, "scan start {i} is negative");
            let at = i as usize;
            assert!(
                at < self.ids.len(),
                "scan index {i} past {} owned ids",
                self.ids.len()
            );
            if self.ids[at] == want {
                return Some(i as u32);
            }
            i = i.wrapping_add(1);
        }
        None
    }
}
