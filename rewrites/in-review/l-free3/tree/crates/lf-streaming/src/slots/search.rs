//! Record search and slot liveness: keys, checked fetch, free tests.
//!
//! Lifted from the verified rewrites. Four small shapes share this
//! module: the record set searched by key (two routines), the word
//! table fetched with a bounds check, the slot in-use bytes tested
//! for freeness, and the slot header initialised in place.

use core::cmp::Ordering;

/// Compares two keys with NUL-terminated semantics.
///
/// Both sides end at the first NUL; a key that ends sorts its
/// terminator against the other side's next byte, exactly as the
/// originals' byte loops do.
fn compare_keys(a: &[u8], b: &[u8]) -> Ordering {
    let mut i = 0;
    loop {
        let ca = a.get(i).copied().unwrap_or(0);
        let cb = b.get(i).copied().unwrap_or(0);
        if ca != cb {
            return ca.cmp(&cb);
        }
        if ca == 0 {
            return Ordering::Equal;
        }
        i += 1;
    }
}

/// A record set: NUL-terminated keys with a 16-bit count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordSet {
    /// Keys in scan order, without their terminators.
    pub keys: Vec<Vec<u8>>,
    /// Count word: zero is empty, 0x8000 and above is unusable.
    pub count: u16,
}

impl RecordSet {
    /// Searches the records for `want`, answering the exact word.
    ///
    /// Restates `stream_record_find_by_key`: 1 on the first match, 0
    /// when the count is zero, and otherwise the last comparison's
    /// sign with its low byte cleared (all high bits set when the
    /// last record's key sorted below the wanted one, else 0). The
    /// answer is exact portable arithmetic, so it is not narrowed.
    /// The record stride is proof planting: the scan reads only the
    /// keys.
    ///
    /// The middle arm below (a count at or above 0x8000 answering
    /// the count with its low byte cleared) is dead, kept because
    /// the original and the rewrite carry it: both zero-extend the
    /// 16-bit count and then compare signed, which only the zero
    /// count satisfies. Counts at or above 0x8000 scan like any
    /// other count.
    ///
    /// # Panics
    ///
    /// When the count names records past the owned keys; the original
    /// reads past its array.
    #[must_use]
    pub fn contains_key(&self, want: &[u8]) -> u32 {
        let count = u32::from(self.count);
        if count == 0 {
            return 0;
        }
        if i32::from(self.count) <= 0 {
            return count & 0xffff_ff00;
        }
        let mut last = Ordering::Equal;
        let mut i = 0;
        while i < count {
            let at = usize::try_from(i).ok().filter(|at| *at < self.keys.len());
            let Some(at) = at else {
                panic!("record index {i} past {} owned keys", self.keys.len());
            };
            last = compare_keys(&self.keys[at], want);
            if last == Ordering::Equal {
                return 1;
            }
            i += 1;
        }
        match last {
            Ordering::Less => 0xffff_ff00,
            Ordering::Equal | Ordering::Greater => 0,
        }
    }

    /// Finds the first record whose key equals `want`.
    ///
    /// Restates `stream_record_lookup`: the record address narrows to
    /// its index (the proof rebuilds it per case). A zero count
    /// matches nothing; counts at or above 0x8000 scan like any
    /// other count (the rewrite's signed check only fires for zero,
    /// because the count is zero-extended first). The record stride
    /// is proof planting: the scan reads only the keys.
    ///
    /// # Panics
    ///
    /// When the count names records past the owned keys; the original
    /// reads past its array.
    #[must_use]
    pub fn position(&self, want: &[u8]) -> Option<usize> {
        let count = u32::from(self.count);
        if i32::from(self.count) <= 0 {
            return None;
        }
        let mut i = 0;
        while i < count {
            let at = usize::try_from(i).ok().filter(|at| *at < self.keys.len());
            let Some(at) = at else {
                panic!("record index {i} past {} owned keys", self.keys.len());
            };
            if compare_keys(&self.keys[at], want) == Ordering::Equal {
                return Some(at);
            }
            i += 1;
        }
        None
    }
}

/// A word table with a 16-bit count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordTable {
    /// Words in index order.
    pub words: Vec<u32>,
    /// Count word, read signed by the check.
    pub count: u16,
}

impl WordTable {
    /// Fetches the `index`-th word, or `None` past the count.
    ///
    /// Restates `stream_table_get_checked`: the index must be below
    /// the count, compared signed. The null answer narrows to `None`
    /// (a stored zero still answers `Some(0)`).
    ///
    /// # Panics
    ///
    /// When the index is negative or names a word past the owned
    /// words; the original reads below or past its array.
    #[must_use]
    // The wrap is the behaviour: the original compares signed.
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    pub fn get(&self, index: u32) -> Option<u32> {
        let i = index as i32;
        if i >= i32::from(self.count) {
            return None;
        }
        assert!(i >= 0, "table index {i} is negative");
        let at = i as usize;
        assert!(
            at < self.words.len(),
            "table index {i} past {} owned words",
            self.words.len()
        );
        Some(self.words[at])
    }
}

/// Slot in-use bytes: 0 is free.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotLiveness {
    /// In-use bytes in slot order.
    pub used: Vec<u8>,
}

/// Index limit when the mode byte selects the small set.
const SMALL_LIMIT: u32 = 15;
/// Index limit when the mode byte selects the big set.
const BIG_LIMIT: u32 = 75;

impl SlotLiveness {
    /// Tests whether slot `index` is free.
    ///
    /// Restates `stream_slot_free`: free when the index is
    /// non-negative, below the limit (15 when the mode flag's low
    /// byte is non-zero, 75 when it is zero), and the slot's in-use
    /// byte is 0. The stride-0x134 global row narrows to dense
    /// bytes. The 1/0 answer narrows to bool.
    ///
    /// # Panics
    ///
    /// When the index names a byte past the owned bytes; the original
    /// reads past its table.
    #[must_use]
    // The wrap is the behaviour: the original compares signed.
    #[allow(clippy::cast_possible_wrap)]
    pub fn is_free(&self, index: u32, flag: u32) -> bool {
        let limit = if flag & 0xff != 0 {
            SMALL_LIMIT
        } else {
            BIG_LIMIT
        };
        if (index as i32) < 0 || index >= limit {
            return false;
        }
        let at = index as usize;
        assert!(
            at < self.used.len(),
            "slot index {index} past {} owned bytes",
            self.used.len()
        );
        self.used[at] == 0
    }
}

/// One slot header: 12 bytes holding a zero word, a flag and a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotHeader {
    /// The word at `+0x00`: zeroed by the initialiser.
    pub zero: u32,
    /// The flag byte at `+0x04`: set to 1 by the initialiser.
    pub flag: u8,
    /// The untouched bytes at `+0x05` through `+0x07`, preserved as read.
    pub pad: [u8; 3],
    /// The value word at `+0x08`.
    pub value: u32,
}

/// Byte length of one slot header.
pub const SLOT_HEADER_LEN: usize = 12;

impl SlotHeader {
    /// Decodes a header from its 12 bytes, little-endian.
    #[must_use]
    pub fn from_bytes(bytes: [u8; SLOT_HEADER_LEN]) -> Self {
        Self {
            zero: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            flag: bytes[4],
            pad: [bytes[5], bytes[6], bytes[7]],
            value: u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
        }
    }

    /// Encodes this header as its 12 bytes, little-endian.
    #[must_use]
    pub fn to_bytes(self) -> [u8; SLOT_HEADER_LEN] {
        let mut out = [0u8; SLOT_HEADER_LEN];
        out[0..4].copy_from_slice(&self.zero.to_le_bytes());
        out[4] = self.flag;
        out[5..8].copy_from_slice(&self.pad);
        out[8..12].copy_from_slice(&self.value.to_le_bytes());
        out
    }

    /// Initialises this header with `value`.
    ///
    /// Restates `stream_slot_init`: zeroes the word at `+0x00`, sets
    /// the flag byte at `+0x04` to 1, and stores `value` at `+0x08`.
    /// The 0 answer narrows to unit.
    pub fn init(&mut self, value: u32) {
        self.zero = 0;
        self.flag = 1;
        self.value = value;
    }
}
