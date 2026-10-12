//! Copies of three or four consecutive words between an object and a caller's
//! block of words.
//!
//! Many routines of the original move a small vector (three words, sometimes
//! a fourth) between a field run inside an object and a buffer the caller
//! passes. Each copy is a run of words that starts at a byte offset in the
//! object and moves in one direction. The routines differ in the direction,
//! the offset and the word they return, and that is what [`WordBlockCopy`]
//! holds. The routines whose copies are not contiguous, or that read or write
//! more than one caller buffer, are not in this module.
//!
//! The object and the buffer are seen as bytes. A word is four bytes, little
//! endian, so a copy of words is a copy of bytes in the same order. No address
//! appears here, and nothing depends on pointer width.
//!
//! Specification of [`WordBlockCopy::run`]:
//!
//! - Word `i` of the run (zero based, `i < words`) lives at byte offset
//!   `object_offset + 4 * i` in the object and at byte offset `4 * i` in the
//!   buffer.
//! - [`CopyDirection::FromObject`] copies each object word into the buffer.
//!   [`CopyDirection::IntoObject`] copies each buffer word into the object.
//!   Nothing else is written.
//! - The outcome is what the original returns: [`CopyOutcome::Null`] when it
//!   returns zero, [`CopyOutcome::Buffer`] when it returns the address of the
//!   buffer it was given, and [`CopyOutcome::Word`] with the value of the last
//!   word of the run when it returns that value.

/// Bytes in one word.
const WORD_BYTES: usize = 4;

/// Which side of the copy is the object.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CopyDirection {
    /// The words are read from the object and written to the buffer.
    FromObject,
    /// The words are read from the buffer and written to the object.
    IntoObject,
}

/// What the routine returns after the copy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CopyReturn {
    /// Zero.
    Null,
    /// The address of the buffer the caller passed.
    Buffer,
    /// The value of the last word of the run.
    LastWord,
}

/// The value a routine returns after a copy, before any address is formed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CopyOutcome {
    /// The routine returned zero.
    Null,
    /// The routine returned the address of the buffer it was given.
    Buffer,
    /// The routine returned this word.
    Word(u32),
}

/// One routine's copy: a run of words, its direction and its return value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WordBlockCopy {
    /// Which side of the copy is the object.
    pub direction: CopyDirection,
    /// Byte offset in the object of the first word of the run.
    pub object_offset: usize,
    /// Number of words in the run: three or four in the family.
    pub words: usize,
    /// What the routine returns.
    pub returns: CopyReturn,
}

impl WordBlockCopy {
    /// A copy with the given direction, object offset, word count and return.
    #[must_use]
    pub const fn new(
        direction: CopyDirection,
        object_offset: usize,
        words: usize,
        returns: CopyReturn,
    ) -> Self {
        Self {
            direction,
            object_offset,
            words,
            returns,
        }
    }

    /// Performs the copy between `object` and `buffer` and returns the outcome.
    ///
    /// # Panics
    ///
    /// When `words` is zero, or when the run lies outside `object` or outside
    /// `buffer`. The original would touch memory beyond the object or the
    /// buffer; the lift stops instead.
    pub fn run(&self, object: &mut [u8], buffer: &mut [u8]) -> CopyOutcome {
        assert!(self.words > 0, "a copy moves at least one word");
        let block_bytes = self.words * WORD_BYTES;
        let object_end = self
            .object_offset
            .checked_add(block_bytes)
            .filter(|&end| end <= object.len())
            .expect("the copy's object run lies outside the object");
        assert!(
            block_bytes <= buffer.len(),
            "the copy's buffer run lies outside the buffer"
        );
        let object_run = &mut object[self.object_offset..object_end];
        let buffer_run = &mut buffer[..block_bytes];
        match self.direction {
            CopyDirection::FromObject => buffer_run.copy_from_slice(object_run),
            CopyDirection::IntoObject => object_run.copy_from_slice(buffer_run),
        }
        match self.returns {
            CopyReturn::Null => CopyOutcome::Null,
            CopyReturn::Buffer => CopyOutcome::Buffer,
            CopyReturn::LastWord => {
                let last = (self.words - 1) * WORD_BYTES;
                let mut bytes = [0u8; WORD_BYTES];
                bytes.copy_from_slice(&buffer_run[last..last + WORD_BYTES]);
                CopyOutcome::Word(u32::from_le_bytes(bytes))
            }
        }
    }
}

/// The copies of the family, one entry per routine that was lifted. The test
/// crate `lf-blockdiff` keeps the mapping from each entry to its rewrite.
pub const BLOCK_COPIES: [WordBlockCopy; 22] = [
    WordBlockCopy::new(CopyDirection::FromObject, 0x10, 3, CopyReturn::LastWord),
    WordBlockCopy::new(CopyDirection::FromObject, 0x18, 3, CopyReturn::LastWord),
    WordBlockCopy::new(CopyDirection::FromObject, 0xa0, 3, CopyReturn::Buffer),
    WordBlockCopy::new(CopyDirection::FromObject, 0x1450, 3, CopyReturn::Buffer),
    WordBlockCopy::new(CopyDirection::FromObject, 0x0c, 3, CopyReturn::Buffer),
    WordBlockCopy::new(CopyDirection::FromObject, 0x2c, 3, CopyReturn::Buffer),
    WordBlockCopy::new(CopyDirection::FromObject, 0x08, 3, CopyReturn::LastWord),
    WordBlockCopy::new(CopyDirection::FromObject, 0x0c, 3, CopyReturn::LastWord),
    WordBlockCopy::new(CopyDirection::FromObject, 0x2c, 3, CopyReturn::LastWord),
    WordBlockCopy::new(CopyDirection::FromObject, 0x14, 3, CopyReturn::Null),
    WordBlockCopy::new(CopyDirection::FromObject, 0x34, 3, CopyReturn::LastWord),
    WordBlockCopy::new(CopyDirection::FromObject, 0x90, 3, CopyReturn::Buffer),
    WordBlockCopy::new(CopyDirection::FromObject, 0x20, 3, CopyReturn::Buffer),
    WordBlockCopy::new(CopyDirection::IntoObject, 0x18, 3, CopyReturn::Null),
    WordBlockCopy::new(CopyDirection::IntoObject, 0x14, 3, CopyReturn::Null),
    WordBlockCopy::new(CopyDirection::IntoObject, 0x34, 3, CopyReturn::LastWord),
    WordBlockCopy::new(CopyDirection::IntoObject, 0x08, 3, CopyReturn::LastWord),
    WordBlockCopy::new(CopyDirection::IntoObject, 0x0c, 3, CopyReturn::LastWord),
    WordBlockCopy::new(CopyDirection::IntoObject, 0x24, 3, CopyReturn::LastWord),
    WordBlockCopy::new(CopyDirection::IntoObject, 0x10, 3, CopyReturn::LastWord),
    WordBlockCopy::new(CopyDirection::IntoObject, 0x60, 4, CopyReturn::LastWord),
    WordBlockCopy::new(CopyDirection::IntoObject, 0x50, 4, CopyReturn::LastWord),
];

#[cfg(test)]
mod tests {
    use super::{BLOCK_COPIES, CopyDirection, CopyOutcome, CopyReturn, WordBlockCopy};

    #[test]
    fn from_object_copies_the_run_into_the_buffer_and_returns_the_last_word() {
        let copy = WordBlockCopy::new(CopyDirection::FromObject, 4, 3, CopyReturn::LastWord);
        let mut object = vec![0u8; 32];
        object[4..8].copy_from_slice(&1u32.to_le_bytes());
        object[8..12].copy_from_slice(&2u32.to_le_bytes());
        object[12..16].copy_from_slice(&0xDEAD_BEEFu32.to_le_bytes());
        let mut buffer = vec![0xEEu8; 16];
        let outcome = copy.run(&mut object, &mut buffer);
        assert_eq!(outcome, CopyOutcome::Word(0xDEAD_BEEF));
        assert_eq!(&buffer[0..4], &1u32.to_le_bytes());
        assert_eq!(&buffer[8..12], &0xDEAD_BEEFu32.to_le_bytes());
        assert_eq!(&buffer[12..16], &[0xEE; 4]);
    }

    #[test]
    fn into_object_writes_only_the_run() {
        let copy = WordBlockCopy::new(CopyDirection::IntoObject, 0, 4, CopyReturn::Null);
        let mut object = vec![0x55u8; 24];
        let mut buffer = vec![0u8; 16];
        buffer[12..16].copy_from_slice(&7u32.to_le_bytes());
        assert_eq!(copy.run(&mut object, &mut buffer), CopyOutcome::Null);
        assert_eq!(&object[12..16], &7u32.to_le_bytes());
        assert_eq!(&object[16..24], &[0x55; 8]);
    }

    #[test]
    fn buffer_return_is_reported_as_the_buffer() {
        let copy = WordBlockCopy::new(CopyDirection::FromObject, 0, 3, CopyReturn::Buffer);
        let mut object = vec![0u8; 16];
        let mut buffer = vec![0u8; 12];
        assert_eq!(copy.run(&mut object, &mut buffer), CopyOutcome::Buffer);
    }

    #[test]
    #[should_panic(expected = "outside the object")]
    fn a_run_past_the_object_stops_the_lift() {
        let copy = WordBlockCopy::new(CopyDirection::FromObject, 0x14, 3, CopyReturn::Null);
        let mut object = vec![0u8; 0x1f];
        let mut buffer = vec![0u8; 16];
        let _ = copy.run(&mut object, &mut buffer);
    }

    #[test]
    #[should_panic(expected = "outside the buffer")]
    fn a_run_past_the_buffer_stops_the_lift() {
        let copy = WordBlockCopy::new(CopyDirection::IntoObject, 0, 4, CopyReturn::Null);
        let mut object = vec![0u8; 32];
        let mut buffer = vec![0u8; 12];
        let _ = copy.run(&mut object, &mut buffer);
    }

    #[test]
    fn the_table_has_the_family_shape() {
        assert_eq!(BLOCK_COPIES.len(), 22);
        assert!(BLOCK_COPIES.iter().all(|c| c.words == 3 || c.words == 4));
        assert_eq!(BLOCK_COPIES.iter().filter(|c| c.words == 4).count(), 2);
    }
}
