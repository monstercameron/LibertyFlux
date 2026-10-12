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

/// One 20-byte entry: the words the routines read and write, plus the
/// bytes both leave untouched (kept so every effect compares).
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
    /// Byte at +0x11: zeroed by the initialiser's word write, untouched
    /// by the find-and-mark.
    pub flag_hi: u8,
    /// Untouched bytes at +18..+19.
    pub tail: [u8; 2],
}

impl Entry {
    /// Reads one entry from 20 bytes.
    ///
    /// # Panics
    ///
    /// When fewer than 20 bytes are given.
    #[must_use]
    pub fn from_bytes(b: &[u8]) -> Self {
        Self {
            ptr: u32::from_le_bytes(b[0..4].try_into().unwrap()),
            tag: u16::from_le_bytes(b[4..6].try_into().unwrap()),
            gap: u16::from_le_bytes(b[6..8].try_into().unwrap()),
            key: u32::from_le_bytes(b[8..12].try_into().unwrap()),
            sum: u32::from_le_bytes(b[12..16].try_into().unwrap()),
            flag: b[16],
            flag_hi: b[17],
            tail: b[18..20].try_into().unwrap(),
        }
    }

    /// Writes the entry back into 20 bytes.
    ///
    /// # Panics
    ///
    /// When fewer than 20 bytes are given.
    pub fn to_bytes(&self, b: &mut [u8]) {
        b[0..4].copy_from_slice(&self.ptr.to_le_bytes());
        b[4..6].copy_from_slice(&self.tag.to_le_bytes());
        b[6..8].copy_from_slice(&self.gap.to_le_bytes());
        b[8..12].copy_from_slice(&self.key.to_le_bytes());
        b[12..16].copy_from_slice(&self.sum.to_le_bytes());
        b[16] = self.flag;
        b[17] = self.flag_hi;
        b[18..20].copy_from_slice(&self.tail);
    }
}

/// The 0x400-entry table of 20-byte entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryTable {
    /// The entries in order.
    pub entries: Vec<Entry>,
}

impl EntryTable {
    /// A table over `ENTRY_COUNT * ENTRY_LEN` bytes.
    ///
    /// # Panics
    ///
    /// When the image is not exactly the table size.
    #[must_use]
    pub fn from_bytes(img: &[u8]) -> Self {
        assert!(
            img.len() == ENTRY_COUNT * ENTRY_LEN,
            "entry table image holds {} bytes, want {}",
            img.len(),
            ENTRY_COUNT * ENTRY_LEN
        );
        let mut entries = Vec::with_capacity(ENTRY_COUNT);
        for chunk in img.chunks_exact(ENTRY_LEN) {
            entries.push(Entry::from_bytes(chunk));
        }
        Self { entries }
    }

    /// Writes every entry back into the image.
    ///
    /// # Panics
    ///
    /// When the image is not exactly the table size.
    pub fn to_bytes(&self, img: &mut [u8]) {
        assert!(
            img.len() == ENTRY_COUNT * ENTRY_LEN,
            "entry table image holds {} bytes, want {}",
            img.len(),
            ENTRY_COUNT * ENTRY_LEN
        );
        for (entry, chunk) in self.entries.iter().zip(img.chunks_exact_mut(ENTRY_LEN)) {
            entry.to_bytes(chunk);
        }
    }

    /// Initialises every entry: pointer 0, tag `0xFFFF`, key and sum 0,
    /// flag word 0; gap and tail bytes untouched. Answers the end offset
    /// plus [`INIT_END_BIAS`], as the 32-bit form answers end + 8.
    pub fn init(&mut self) -> usize {
        for entry in &mut self.entries {
            entry.ptr = 0;
            entry.tag = INIT_TAG;
            entry.key = 0;
            entry.sum = 0;
            entry.flag = 0;
            entry.flag_hi = 0;
        }
        self.entries.len() * ENTRY_LEN + INIT_END_BIAS
    }

    /// Finds the first entry whose pointer is non-null, whose key equals
    /// `key` and whose sum equals `base + add` (wrapping), sets its flag
    /// byte to 1 and answers its index. Answers `None` when nothing
    /// matches (the 32-bit form answers [`FIND_MISS`], and the index times
    /// [`FIND_SCALE`] on a hit).
    pub fn find_mark(&mut self, key: u32, base: u32, add: u32) -> Option<usize> {
        let target = base.wrapping_add(add);
        for (i, entry) in self.entries.iter_mut().enumerate() {
            if entry.ptr != 0 && entry.key == key && entry.sum == target {
                entry.flag = 1;
                return Some(i);
            }
        }
        None
    }
}

/// Entries in the 44-byte revocation table.
pub const REVOC_COUNT: usize = 1100;
/// Bytes per entry of the revocation table.
pub const REVOC_LEN: usize = 44;
/// Flag bit the reset and clear drop, and the revoke tests.
pub const REVOC_BIT: u8 = 0x10;

/// One 44-byte entry: the tag word at +0 and the body, whose bytes at
/// +9 (flag) and +10 decide a revocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevocEntry {
    /// Tag word at +0.
    pub tag: u16,
    /// The remaining 42 bytes; body[7] is the flag byte (entry +9).
    pub body: [u8; 42],
}

impl RevocEntry {
    /// Reads one entry from 44 bytes.
    ///
    /// # Panics
    ///
    /// When fewer than 44 bytes are given.
    #[must_use]
    pub fn from_bytes(b: &[u8]) -> Self {
        Self {
            tag: u16::from_le_bytes(b[0..2].try_into().unwrap()),
            body: b[2..44].try_into().unwrap(),
        }
    }

    /// Writes the entry back into 44 bytes.
    ///
    /// # Panics
    ///
    /// When fewer than 44 bytes are given.
    pub fn to_bytes(&self, b: &mut [u8]) {
        b[0..2].copy_from_slice(&self.tag.to_le_bytes());
        b[2..44].copy_from_slice(&self.body);
    }

    /// The flag byte (entry +9).
    #[must_use]
    pub fn flag(&self) -> u8 {
        self.body[7]
    }

    /// The byte past the flag (entry +10).
    #[must_use]
    pub fn next(&self) -> u8 {
        self.body[8]
    }
}

/// Resets the revocation table's slots: the reset routine's callee (its
/// answer is ignored).
pub trait SlotReset {
    /// Runs the slot reset.
    fn reset_slots(&mut self);
}

impl<F: FnMut()> SlotReset for F {
    fn reset_slots(&mut self) {
        self();
    }
}

/// The 44-byte revocation table with its header words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevocTable {
    /// First header word: zeroed by reset/clear, the live count for revoke.
    pub head_a: u32,
    /// Second header word.
    pub head_b: u32,
    /// Four header bytes.
    pub head_c: [u8; 4],
    /// Fourth header word.
    pub head_d: u32,
    /// The entries in order.
    pub entries: Vec<RevocEntry>,
}

impl RevocTable {
    /// A table over its header words and `REVOC_COUNT * REVOC_LEN` bytes.
    ///
    /// # Panics
    ///
    /// When the image is not exactly the table size.
    #[must_use]
    pub fn from_parts(head_a: u32, head_b: u32, head_c: [u8; 4], head_d: u32, img: &[u8]) -> Self {
        assert!(
            img.len() == REVOC_COUNT * REVOC_LEN,
            "revocation image holds {} bytes, want {}",
            img.len(),
            REVOC_COUNT * REVOC_LEN
        );
        let mut entries = Vec::with_capacity(REVOC_COUNT);
        for chunk in img.chunks_exact(REVOC_LEN) {
            entries.push(RevocEntry::from_bytes(chunk));
        }
        Self {
            head_a,
            head_b,
            head_c,
            head_d,
            entries,
        }
    }

    /// Writes every entry back into the image.
    ///
    /// # Panics
    ///
    /// When the image is not exactly the table size.
    pub fn to_bytes(&self, img: &mut [u8]) {
        assert!(
            img.len() == REVOC_COUNT * REVOC_LEN,
            "revocation image holds {} bytes, want {}",
            img.len(),
            REVOC_COUNT * REVOC_LEN
        );
        for (entry, chunk) in self.entries.iter().zip(img.chunks_exact_mut(REVOC_LEN)) {
            entry.to_bytes(chunk);
        }
    }

    /// End offset of the table: what the routines' final cursor narrows to
    /// (the 32-bit form answers the flag cursor, 9 past this per entry).
    #[must_use]
    pub fn end_offset(&self) -> usize {
        self.entries.len() * REVOC_LEN
    }

    /// Resets the table: zeroes the first two header words, runs the slot
    /// reset, and clears [`REVOC_BIT`] in every flag byte.
    pub fn reset(&mut self, slots: &mut impl SlotReset) -> usize {
        self.head_a = 0;
        self.head_b = 0;
        slots.reset_slots();
        for entry in &mut self.entries {
            entry.body[7] &= !REVOC_BIT;
        }
        self.end_offset()
    }

    /// Clears the table: zeroes the first two header words and the four
    /// header bytes, clears [`REVOC_BIT`] in every flag byte, then zeroes
    /// the fourth header word.
    pub fn clear(&mut self) -> usize {
        self.head_a = 0;
        self.head_b = 0;
        self.head_c = [0; 4];
        for entry in &mut self.entries {
            entry.body[7] &= !REVOC_BIT;
        }
        self.head_d = 0;
        self.end_offset()
    }

    /// Revokes every entry tagged with the low 16 bits of `key` whose flag
    /// byte has [`REVOC_BIT`] set and whose next byte has it clear:
    /// clears the bit, zeroes the tag, and decrements the live count
    /// (wrapping) per revocation.
    pub fn revoke(&mut self, key: u32) -> usize {
        // The low 16 bits are the tag by definition.
        #[allow(clippy::cast_possible_truncation)]
        let tag = key as u16;
        let mut live = self.head_a;
        for entry in &mut self.entries {
            if entry.tag == tag && entry.flag() & REVOC_BIT != 0 && entry.next() & REVOC_BIT == 0 {
                entry.body[7] &= !REVOC_BIT;
                entry.tag = 0;
                live = live.wrapping_sub(1);
            }
        }
        self.head_a = live;
        self.end_offset()
    }
}
