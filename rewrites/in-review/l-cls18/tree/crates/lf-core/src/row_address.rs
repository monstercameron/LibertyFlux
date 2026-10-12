//! Row addresses: the address of one row of a table, computed from a base, an index and a row size.
//!
//! Several small routines of the original return the address of one row of a table: the base, plus the index
//! times the row size, plus a fixed offset within the row. The base is either the address of a global table, or
//! the pointer stored in a global. The index is either a parameter or a word of the routine's object. The
//! routines differ in those sources and in the constants. [`RowAddress`] holds one such routine as data, and
//! [`RowAddress::apply`] computes the address once the caller has resolved the base and read the index.
//!
//! Specification of [`RowAddress::apply`]:
//!
//! - The index is the parameter, or the word of the object at the field's offset, as four bytes least
//!   significant first.
//! - The result is `base + index * stride + offset` in 32-bit wrapping arithmetic.
//! - Nothing is read or written through the result: it is an address, not a value.
//! - The caller resolves the base: for [`BaseSource::Global`] it is the global's own address, and for
//!   [`BaseSource::PointerInGlobal`] it is the word stored at the global.

use crate::field_store::ObjectBytes;

/// Where the base of a row comes from. The caller resolves it and passes the result to [`RowAddress::apply`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BaseSource {
    /// The table is the global itself: its base is the global's address.
    Global,
    /// The global holds a pointer to the table: its base is the word stored at the global.
    PointerInGlobal,
}

/// Where the index of a row comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexSource {
    /// The routine's own parameter.
    Parameter,
    /// A 32-bit word of the object, at this byte offset.
    Field(usize),
}

/// One row-address routine, as data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowAddress {
    /// Where the base comes from.
    pub base: BaseSource,
    /// Where the index comes from.
    pub index: IndexSource,
    /// Bytes between two consecutive rows.
    pub stride: u32,
    /// Bytes from the start of the row to the address returned.
    pub offset: u32,
}

impl RowAddress {
    /// The address of the row, for a base the caller has resolved, the object, and the parameter.
    ///
    /// # Panics
    ///
    /// When the index field lies outside the object.
    #[must_use]
    pub fn apply(&self, base: u32, object: &ObjectBytes, parameter: u32) -> u32 {
        let index = match self.index {
            IndexSource::Parameter => parameter,
            IndexSource::Field(offset) => object.word(offset),
        };
        base.wrapping_add(index.wrapping_mul(self.stride))
            .wrapping_add(self.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::{BaseSource, IndexSource, RowAddress};
    use crate::field_store::ObjectBytes;

    #[test]
    fn the_row_is_base_plus_index_times_stride_plus_offset() {
        let row = RowAddress {
            base: BaseSource::Global,
            index: IndexSource::Parameter,
            stride: 0x58,
            offset: 0x20,
        };
        let object = ObjectBytes::zeroed(4);
        assert_eq!(row.apply(0x1000, &object, 0), 0x1020);
        assert_eq!(row.apply(0x1000, &object, 3), 0x1000 + 3 * 0x58 + 0x20);
    }

    #[test]
    fn a_field_index_is_read_from_the_object() {
        let row = RowAddress {
            base: BaseSource::PointerInGlobal,
            index: IndexSource::Field(4),
            stride: 16,
            offset: 0,
        };
        let mut object = ObjectBytes::zeroed(8);
        object.write(4, &5u32.to_le_bytes());
        assert_eq!(row.apply(0x2000, &object, 99), 0x2000 + 5 * 16);
    }

    #[test]
    fn the_sum_wraps_as_a_thirty_two_bit_address_does() {
        let row = RowAddress {
            base: BaseSource::Global,
            index: IndexSource::Parameter,
            stride: 0x10,
            offset: 0,
        };
        let object = ObjectBytes::zeroed(4);
        assert_eq!(row.apply(u32::MAX, &object, 1), 0x0F);
    }
}
