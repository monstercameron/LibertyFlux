//! The task state block: one header word and nine records of eight words,
//! seventy-three words in all, with the reset that marks every record's
//! two handle fields invalid.
//!
//! The original clears the whole block and then writes the invalid-handle
//! sentinel into the first two words of each record. Its stores come in a
//! scattered order with the header last; only the final contents are
//! observable, so [`TaskStateBlock::reset`] states the final contents
//! directly. The owning type is [`TaskStateBlock`]: the header word, then
//! [`StateRecord`]s, each with its two handle words and six further words.

/// Records in the block.
pub const STATE_RECORD_COUNT: usize = 9;
/// Words in one record.
pub const STATE_RECORD_WORDS: usize = 8;
/// Words in the whole block: the header and every record.
pub const BLOCK_WORDS: usize = 1 + STATE_RECORD_COUNT * STATE_RECORD_WORDS;
/// The invalid-handle sentinel that [`TaskStateBlock::reset`] writes into
/// each record's two handle words.
pub const INVALID_HANDLE: u32 = 0xFFFF_FFFF;

/// One record of the block: the two handle words, then six further words.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StateRecord {
    /// The first handle word (word 1 of the record, 4 bytes into it).
    pub handle_a: u32,
    /// The second handle word (word 2 of the record).
    pub handle_b: u32,
    /// The six remaining words of the record, in order.
    pub words: [u32; STATE_RECORD_WORDS - 2],
}

/// The task state block, owned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskStateBlock {
    /// The header word, at the start of the block.
    pub header: u32,
    /// The nine records that follow the header.
    pub records: [StateRecord; STATE_RECORD_COUNT],
}

impl Default for TaskStateBlock {
    fn default() -> Self {
        Self {
            header: 0,
            records: [StateRecord::default(); STATE_RECORD_COUNT],
        }
    }
}

impl TaskStateBlock {
    /// Clears every word of the block, then marks each record's two handle
    /// words invalid.
    pub fn reset(&mut self) {
        *self = Self::default();
        for record in &mut self.records {
            record.handle_a = INVALID_HANDLE;
            record.handle_b = INVALID_HANDLE;
        }
    }

    /// The block's words in memory order: the header, then each record's
    /// two handle words and six further words.
    #[must_use]
    pub fn to_words(&self) -> [u32; BLOCK_WORDS] {
        let mut out = [0_u32; BLOCK_WORDS];
        out[0] = self.header;
        for (k, record) in self.records.iter().enumerate() {
            let base = 1 + k * STATE_RECORD_WORDS;
            out[base] = record.handle_a;
            out[base + 1] = record.handle_b;
            out[base + 2..base + STATE_RECORD_WORDS].copy_from_slice(&record.words);
        }
        out
    }
}
