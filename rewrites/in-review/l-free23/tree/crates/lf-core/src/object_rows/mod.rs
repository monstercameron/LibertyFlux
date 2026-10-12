//! Row lookups in the words of an object.
//!
//! Several records of the original keep a row flag word and a row base word.
//! The base names the first element of an array of fixed-size rows, and the
//! flag says whether the array is present. A lookup returns the address of
//! the row for an index, or the null address when the flag is clear. Each
//! record type uses its own word offsets and row size, so the same routine
//! serves all of them, and the parameters live in a table of instances.
//!
//! The object is seen as its bytes: a word is four bytes, little endian, at a
//! byte offset from the start of the object. Nothing here dereferences the
//! base word. It is a number, and the result is a number, so no address
//! appears in this module and nothing depends on pointer width.
//!
//! Specification of [`RowLookup::row_or_null`]:
//!
//! - The flag word is read at `flag_offset`. When it is zero the result is
//!   zero, and the base word is not read at all.
//! - Otherwise the result is the base word plus `index` times the row stride,
//!   in wrapping 32-bit arithmetic. The index is not bounds checked, so an
//!   index past the end of the array, or one that wraps, gives the value the
//!   same arithmetic gives in the original.
//! - The object is only read. Nothing is written.
//!
//! [`ROW_LOOKUPS`] lists the parameter sets of the row lookups that were
//! lifted from verified rewrites. Each entry names the offsets and stride that
//! one rewrite fixes as constants; the test crate that proves them keeps the
//! mapping from each entry to its rewrite.

/// The word offsets and row stride of one row lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RowLookup {
    /// Byte offset of the row flag word in the object.
    pub flag_offset: usize,
    /// Byte offset of the row base word in the object.
    pub base_offset: usize,
    /// Size in bytes of one row. The index is multiplied by this.
    pub stride: u32,
}

impl RowLookup {
    /// A lookup with the given offsets and row stride.
    #[must_use]
    pub const fn new(flag_offset: usize, base_offset: usize, stride: u32) -> Self {
        Self {
            flag_offset,
            base_offset,
            stride,
        }
    }

    /// The address of row `index` in the object's array, or zero when the
    /// flag word is zero.
    ///
    /// # Panics
    ///
    /// When a word the lookup reads lies outside `object`. The original would
    /// read memory past the record; the lift stops instead.
    #[must_use]
    pub fn row_or_null(&self, object: &[u8], index: u32) -> u32 {
        if read_word(object, self.flag_offset) == 0 {
            return 0;
        }
        read_word(object, self.base_offset).wrapping_add(index.wrapping_mul(self.stride))
    }
}

/// The parameter sets of the row lookups lifted from verified rewrites.
///
/// Each entry is one rewrite's constants: the flag and base offsets and the
/// row stride. The lookup reads the same words, in the same order, as the
/// rewrite does.
pub const ROW_LOOKUPS: [RowLookup; 10] = [
    RowLookup::new(0x0c, 0x44, 20),
    RowLookup::new(0x0c, 0x30, 8),
    RowLookup::new(0x0c, 0x30, 20),
    RowLookup::new(0x0c, 0x4c, 20),
    RowLookup::new(0x0c, 0x20, 24),
    RowLookup::new(0x08, 0x40, 20),
    RowLookup::new(0x08, 0x2c, 8),
    RowLookup::new(0x08, 0x2c, 20),
    RowLookup::new(0x08, 0x48, 20),
    RowLookup::new(0x08, 0x1c, 24),
];

/// The four-byte little-endian word at `offset` in `object`.
///
/// # Panics
///
/// When the word does not lie wholly inside `object`.
#[must_use]
pub fn read_word(object: &[u8], offset: usize) -> u32 {
    let end = offset
        .checked_add(4)
        .filter(|&end| end <= object.len())
        .expect("object word lies outside the object");
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&object[offset..end]);
    u32::from_le_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::{RowLookup, ROW_LOOKUPS, read_word};

    fn object_with(flag_offset: usize, flag: u32, base_offset: usize, base: u32) -> Vec<u8> {
        let mut object = vec![0u8; 0x60];
        object[flag_offset..flag_offset + 4].copy_from_slice(&flag.to_le_bytes());
        object[base_offset..base_offset + 4].copy_from_slice(&base.to_le_bytes());
        object
    }

    #[test]
    fn clear_flag_gives_null_whatever_the_index() {
        let lookup = RowLookup::new(0x0c, 0x44, 20);
        let object = object_with(0x0c, 0, 0x44, 0x1234_5678);
        assert_eq!(lookup.row_or_null(&object, 0), 0);
        assert_eq!(lookup.row_or_null(&object, 7), 0);
        assert_eq!(lookup.row_or_null(&object, u32::MAX), 0);
    }

    #[test]
    fn set_flag_gives_base_plus_index_times_stride() {
        let lookup = RowLookup::new(0x0c, 0x44, 20);
        let object = object_with(0x0c, 1, 0x44, 0x1000);
        assert_eq!(lookup.row_or_null(&object, 0), 0x1000);
        assert_eq!(lookup.row_or_null(&object, 3), 0x1000 + 60);
    }

    #[test]
    fn index_arithmetic_wraps_like_the_original() {
        let lookup = RowLookup::new(0x08, 0x2c, 24);
        let object = object_with(0x08, 5, 0x2c, 0xFFFF_FFF0);
        assert_eq!(lookup.row_or_null(&object, u32::MAX), 0xFFFF_FFF0u32.wrapping_add(u32::MAX.wrapping_mul(24)));
        assert_eq!(lookup.row_or_null(&object, 0x8000_0000), 0xFFFF_FFF0);
    }

    #[test]
    fn flag_word_is_any_nonzero_value() {
        let lookup = RowLookup::new(0x08, 0x1c, 8);
        let object = object_with(0x08, 0x8000_0000, 0x1c, 0x40);
        assert_eq!(lookup.row_or_null(&object, 2), 0x40 + 16);
    }

    #[test]
    fn the_base_is_not_read_when_the_flag_is_clear() {
        let lookup = RowLookup::new(0x08, 0x5c, 8);
        let object = vec![0u8; 0x08 + 4];
        assert_eq!(lookup.row_or_null(&object, 1), 0);
    }

    #[test]
    fn read_word_is_little_endian() {
        let object = [0x78, 0x56, 0x34, 0x12, 0xAA];
        assert_eq!(read_word(&object, 0), 0x1234_5678);
        assert_eq!(read_word(&object, 1), 0xAA12_3456);
    }

    #[test]
    #[should_panic(expected = "outside the object")]
    fn a_word_past_the_end_stops_the_lift() {
        let lookup = RowLookup::new(0x0c, 0x44, 20);
        let mut object = vec![0u8; 0x20];
        object[0x0c] = 1;
        let _ = lookup.row_or_null(&object, 0);
    }

    #[test]
    fn the_table_entries_are_the_rows_that_were_lifted() {
        assert_eq!(ROW_LOOKUPS.len(), 10);
        assert!(ROW_LOOKUPS.iter().all(|entry| entry.stride != 0));
        assert_eq!(ROW_LOOKUPS[0], RowLookup::new(0x0c, 0x44, 20));
        assert_eq!(ROW_LOOKUPS[9], RowLookup::new(0x08, 0x1c, 24));
    }
}
