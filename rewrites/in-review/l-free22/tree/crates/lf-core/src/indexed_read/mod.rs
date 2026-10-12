//! Indexed reads: a byte or a word of an object, at a fixed base offset plus an index times a fixed stride.
//!
//! Several small routines of the original select one entry of a table that sits inside an object: the entry
//! for an index is at the table's base offset plus the index times the entry size. They read one byte or one
//! 32-bit word there and return it. [`IndexedRead`] describes one such routine as data. [`IndexedRead::read`]
//! is the routine. The instances the verified rewrites were checked against are listed in [`instances`].
//!
//! Specification of [`IndexedRead::read`]:
//!
//! - The offset is computed as the original computes an address: the index times the stride, plus the base,
//!   in 32-bit wrapping arithmetic. [`IndexedRead::offset`] returns it.
//! - A byte read returns that byte as a word. A word read returns the 32-bit word, least significant byte
//!   first, and a floating-point word is returned as its 32 bits.
//! - Nothing else is read, and nothing is written.
//! - An offset whose field does not lie inside the object is a caller error and panics. The original would
//!   have read outside the object; the lift fails loudly instead of guessing. The wrapped offset is what is
//!   checked, so an index that wraps the 32-bit offset is compared with the wrapped value.

use crate::field_store::ObjectBytes;

pub mod instances;

/// The width of the entry an indexed read returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryWidth {
    /// One byte, returned as a word.
    Byte,
    /// One 32-bit word.
    Word,
}

/// One indexed read, as data: the base offset of the table, the entry size and the width read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndexedRead {
    /// Byte offset of the table from the start of the object.
    pub base: u32,
    /// Bytes between two consecutive entries.
    pub stride: u32,
    /// Width of the entry read.
    pub width: EntryWidth,
}

impl IndexedRead {
    /// The byte offset of the entry for `index`, in 32-bit wrapping arithmetic.
    #[must_use]
    pub fn offset(&self, index: u32) -> u32 {
        index.wrapping_mul(self.stride).wrapping_add(self.base)
    }

    /// Reads the entry for `index` from `object`.
    ///
    /// # Panics
    ///
    /// When the entry does not lie inside the object.
    #[must_use]
    pub fn read(&self, object: &ObjectBytes, index: u32) -> u32 {
        let offset = self.offset(index) as usize;
        match self.width {
            EntryWidth::Byte => u32::from(object.byte(offset)),
            EntryWidth::Word => object.word(offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EntryWidth, IndexedRead};
    use crate::field_store::ObjectBytes;

    #[test]
    fn the_entry_for_an_index_is_base_plus_index_times_stride() {
        let read = IndexedRead {
            base: 4,
            stride: 12,
            width: EntryWidth::Word,
        };
        assert_eq!(read.offset(0), 4);
        assert_eq!(read.offset(3), 40);
    }

    #[test]
    fn a_byte_entry_is_returned_as_a_word() {
        let read = IndexedRead {
            base: 2,
            stride: 1,
            width: EntryWidth::Byte,
        };
        let object = ObjectBytes::from_bytes(&[0, 0, 0xFE, 0, 0]);
        assert_eq!(read.read(&object, 0), 0xFE);
    }

    #[test]
    fn the_offset_wraps_as_a_thirty_two_bit_address_does() {
        let read = IndexedRead {
            base: 8,
            stride: 16,
            width: EntryWidth::Word,
        };
        // 0x1000_0000 * 16 is 2^32, which wraps to zero; the entry is then at the base.
        assert_eq!(read.offset(0x1000_0000), 8);
    }

    #[test]
    #[should_panic(expected = "lies outside an object")]
    fn an_entry_past_the_end_panics() {
        let read = IndexedRead {
            base: 0,
            stride: 4,
            width: EntryWidth::Word,
        };
        let object = ObjectBytes::zeroed(8);
        let _ = read.read(&object, 2);
    }
}
