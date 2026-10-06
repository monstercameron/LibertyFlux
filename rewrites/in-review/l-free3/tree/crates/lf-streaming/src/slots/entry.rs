//! One streaming slot-table entry: 24 bytes recording what is loaded.
//!
//! Lifted from the verified rewrites. Word map, read off every routine
//! that touches an entry:
//!
//! | Word | Holds |
//! |---|---|
//! | 0 (`+0x00`) | link word (no lifted routine interprets it) |
//! | 1 (`+0x04`) | value word: low byte selects a data slot, upper 24 bits are added to it |
//! | 2 (`+0x08`) | size in 4-byte units; only bits above the low two count |
//! | 3 (`+0x0c`) | spare word whose upper half is the flag word (`+0x0e`) |
//! | 4 (`+0x10`) | link word, all-ones when empty |
//! | 5 (`+0x14`) | link word, all-ones when empty; top byte is the kind byte (`+0x17`) |
//!
//! An entry is active when the size word has any bit above the low two
//! set or bit 11 of the flag word is set; every active/empty test in the
//! family is that one predicate.

/// Byte length of one entry.
pub const ENTRY_LEN: usize = 24;
/// Size bits that count: everything above the low two.
const SIZE_MASK: u32 = 0xffff_fffc;
/// Flag bit marking an entry present.
const PRESENT_BIT: u32 = 11;
/// Round-up bias of the block count: `ceil(units / 2048)`.
const BLOCK_ROUND_UP: u32 = 0x7ff;
/// Block-count shift: units per 2K-unit block, as a power of two.
const BLOCK_SHIFT: u32 = 11;
/// Data-slot count: the value word's low byte selects one.
const DATA_SLOT_COUNT: usize = 256;
/// Link words of an empty entry.
const EMPTY_LINK: u32 = 0xffff_ffff;

/// One 24-byte streaming entry, owned as six words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamEntry {
    /// The six entry words in address order.
    words: [u32; 6],
}

impl StreamEntry {
    /// Builds an entry from its six words in address order.
    #[must_use]
    pub const fn from_words(words: [u32; 6]) -> Self {
        Self { words }
    }

    /// The six entry words in address order.
    #[must_use]
    pub const fn words(&self) -> [u32; 6] {
        self.words
    }

    /// An entry in its empty state: four zero words, two all-ones links.
    ///
    /// Restates `stream_entry_init`: the original zeroes the words at
    /// `+0x00` through `+0x0c` and writes all-ones at `+0x10`/`+0x14`,
    /// answering 0.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            words: [0, 0, 0, 0, EMPTY_LINK, EMPTY_LINK],
        }
    }

    /// Resets this entry to its empty state.
    ///
    /// Restates `stream_entry_init` as a mutation; the 0 answer narrows
    /// to unit (every call answers 0).
    pub fn reset(&mut self) {
        *self = Self::empty();
    }

    /// Decodes an entry from its 24 bytes, little-endian.
    #[must_use]
    pub fn from_bytes(bytes: [u8; ENTRY_LEN]) -> Self {
        let mut words = [0u32; 6];
        let mut i = 0;
        while i < 6 {
            let at = i * 4;
            words[i] =
                u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);
            i += 1;
        }
        Self { words }
    }

    /// Encodes this entry as its 24 bytes, little-endian.
    #[must_use]
    pub fn to_bytes(self) -> [u8; ENTRY_LEN] {
        let mut out = [0u8; ENTRY_LEN];
        let mut i = 0;
        while i < 6 {
            let at = i * 4;
            let bytes = self.words[i].to_le_bytes();
            out[at..at + 4].copy_from_slice(&bytes);
            i += 1;
        }
        out
    }

    /// The value word at `+0x04`.
    #[must_use]
    pub const fn value(self) -> u32 {
        self.words[1]
    }

    /// The size word at `+0x08`, in 4-byte units.
    #[must_use]
    pub const fn size_units(self) -> u32 {
        self.words[2]
    }

    /// The flag word at `+0x0e`: the upper half of the word at `+0x0c`.
    #[must_use]
    pub const fn flags(self) -> u16 {
        (self.words[3] >> 16) as u16
    }

    /// The kind byte at `+0x17`: the top byte of the word at `+0x14`.
    #[must_use]
    pub const fn kind(self) -> u8 {
        (self.words[5] >> 24) as u8
    }

    /// The kind byte of the value word: the low byte of `+0x04`.
    #[must_use]
    pub const fn value_kind(self) -> u8 {
        self.words[1] as u8
    }

    /// Whether this entry counts as active.
    ///
    /// Restates `stream_entry_is_active`: active when the size word has
    /// any bit above the low two set or bit 11 of the flag word is set.
    /// The 1/0 answer narrows to bool.
    #[must_use]
    pub const fn is_active(self) -> bool {
        self.words[2] & SIZE_MASK != 0 || (self.flags() as u32 >> PRESENT_BIT) & 1 != 0
    }

    /// Number of whole 2K-unit blocks in the stored size, rounded up.
    ///
    /// Restates `stream_entry_block_count`: `((size >> 2) + 0x7ff) >> 11`
    /// with wrapping addition.
    #[must_use]
    pub const fn block_count(self) -> u32 {
        (self.words[2] >> 2).wrapping_add(BLOCK_ROUND_UP) >> BLOCK_SHIFT
    }

    /// Data address of this entry, or 0 when it is empty.
    ///
    /// Restates `stream_entry_data_address`: empty entries answer 0;
    /// otherwise the value word's low byte selects one of 256 data
    /// slots and the answer is `(value >> 8)` plus the slot, wrapping.
    /// The 32-bit slots sit 160 bytes apart in one global region; the
    /// lift owns the 256 slot values densely.
    #[must_use]
    pub fn data_address(self, slots: &DataSlots) -> u32 {
        if !self.is_active() {
            return 0;
        }
        let value = self.value();
        (value >> 8).wrapping_add(slots.slots[(value & 0xff) as usize])
    }

    /// Locates this entry's data: the address and block count, or `None`.
    ///
    /// Restates `stream_entry_get_location`: empty entries answer 0 and
    /// write nothing; otherwise the data address comes from the
    /// provider and the block count from [`Self::block_count`]. The
    /// 1/0 answer narrows to `Some`/`None`, and the two out words
    /// narrow to the returned pair.
    pub fn locate<P: DataAddress>(&self, provider: &mut P) -> Option<(u32, u32)> {
        if !self.is_active() {
            return None;
        }
        let data = provider.data_address(self);
        Some((data, self.block_count()))
    }

    /// Sets bit 15 of this entry's flag word.
    ///
    /// The flag word is the upper half of the word at `+0x0c`.
    pub fn set_flag_bit15(&mut self) {
        self.words[3] |= 0x8000_0000;
    }
}

/// The 256 data-slot values behind the global data table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSlots {
    /// Slot values in selector order.
    pub slots: [u32; DATA_SLOT_COUNT],
}

impl DataSlots {
    /// Builds the slot table from its 256 values in selector order.
    #[must_use]
    pub const fn from_slots(slots: [u32; DATA_SLOT_COUNT]) -> Self {
        Self { slots }
    }
}

/// The global kind-flag bytes behind the kind table.
///
/// The 32-bit table holds one flag byte per value-kind at kind
/// multiples of 160, so `256 * 160` bytes plus the final flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindBytes {
    /// Flag bytes in table order.
    pub bytes: [u8; KindBytes::LEN],
}

impl KindBytes {
    /// Byte length of the kind table: the last slot plus its flag.
    pub const LEN: usize = (DATA_SLOT_COUNT - 1) * 160 + 1;

    /// Builds the kind table from its bytes in table order.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; KindBytes::LEN]) -> Self {
        Self { bytes }
    }
}

/// The 256 kind base slots behind the slot-index table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindSlots {
    /// Base slots in kind order.
    pub slots: [u32; DATA_SLOT_COUNT],
}

impl KindSlots {
    /// Builds the kind table from its 256 base slots in kind order.
    #[must_use]
    pub const fn from_slots(slots: [u32; DATA_SLOT_COUNT]) -> Self {
        Self { slots }
    }
}

/// Supplies a stream entry's data address: the location callee.
///
/// The 32-bit callee takes the entry address and answers the data
/// address; the lift passes the entry itself.
pub trait DataAddress {
    /// Answers the data address of `entry`.
    fn data_address(&mut self, entry: &StreamEntry) -> u32;
}

impl<F: FnMut(&StreamEntry) -> u32> DataAddress for F {
    fn data_address(&mut self, entry: &StreamEntry) -> u32 {
        self(entry)
    }
}
