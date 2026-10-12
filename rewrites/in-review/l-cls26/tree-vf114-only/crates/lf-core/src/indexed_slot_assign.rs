//! Indexed slot assignment: an index stored in one field of an object, and the table word at that index
//! stored in a second field.
//!
//! Four small routines of the original share one shape. Each is a method of an object. It takes one
//! index, writes the index into an index field of the object, reads the word at that index in a global
//! table, writes the word into a value field of the same object, and returns the word. The two field
//! offsets are the only things that differ between the routines; the table is the same for all four.
//! [`IndexedSlotAssign`] describes one of them as data, and [`IndexedSlotAssign::assign`] is the routine.
//!
//! The table is a slice of words that the caller owns. The lift never names a global address; the caller
//! passes the table in, and the test crate that proves this routine passes the table the verified rewrites
//! read.
//!
//! Specification of [`IndexedSlotAssign::assign`]:
//!
//! - The index is written first, as one word at the index field, least significant byte first.
//! - The table word at `table_index` is read. The table's first word is index zero.
//! - That word is written as one word at the value field.
//! - The routine returns that word. Nothing else of the object is read or written.
//! - An index past the end of the table is a caller error and panics. The original reads memory at that
//!   address without any check; the lift does not reproduce an unchecked read, so the caller must give
//!   the whole table the index can name.
//! - Nothing here loops or allocates.

use crate::field_store::ObjectBytes;

/// One routine of this family: where its index and value fields lie in the object.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndexedSlotAssign {
    /// Byte offset of the index field from the start of the object.
    pub index_offset: usize,
    /// Byte offset of the value field from the start of the object.
    pub value_offset: usize,
}

impl IndexedSlotAssign {
    /// Describes one routine by the offsets of its index field and its value field.
    #[must_use]
    pub const fn new(index_offset: usize, value_offset: usize) -> Self {
        Self {
            index_offset,
            value_offset,
        }
    }

    /// Runs the routine on `object` with `table_index`, reading the word from `table`, and returns the word.
    /// See the module specification.
    ///
    /// # Panics
    ///
    /// When a field lies outside the object, or when `table_index` is not a position in `table`.
    pub fn assign(&self, object: &mut ObjectBytes, table: &[u32], table_index: u32) -> u32 {
        object.write(self.index_offset, &table_index.to_le_bytes());
        let table_value = table[table_index as usize];
        object.write(self.value_offset, &table_value.to_le_bytes());
        table_value
    }
}

#[cfg(test)]
mod tests {
    use super::IndexedSlotAssign;
    use crate::field_store::ObjectBytes;

    #[test]
    fn index_then_value_then_return() {
        let routine = IndexedSlotAssign::new(0x10, 0x20);
        let mut object = ObjectBytes::zeroed(0x40);
        let table = [0xAAAA_0000, 0xBBBB_1111, 0xCCCC_2222];
        let returned = routine.assign(&mut object, &table, 2);
        assert_eq!(returned, 0xCCCC_2222);
        assert_eq!(object.word(0x10), 2);
        assert_eq!(object.word(0x20), 0xCCCC_2222);
    }

    #[test]
    fn only_the_two_fields_change() {
        let routine = IndexedSlotAssign::new(0x4, 0x8);
        let mut object = ObjectBytes::from_bytes(&[0xEE; 0x10]);
        let table = [0x1234_5678];
        routine.assign(&mut object, &table, 0);
        assert_eq!(object.word(0x0), 0xEEEE_EEEE);
        assert_eq!(object.word(0xC), 0xEEEE_EEEE);
        assert_eq!(object.word(0x4), 0);
        assert_eq!(object.word(0x8), 0x1234_5678);
    }

    #[test]
    #[should_panic]
    fn an_index_past_the_table_panics() {
        let routine = IndexedSlotAssign::new(0x0, 0x4);
        let mut object = ObjectBytes::zeroed(0x8);
        routine.assign(&mut object, &[7], 1);
    }

    #[test]
    #[should_panic]
    fn a_field_outside_the_object_panics() {
        let routine = IndexedSlotAssign::new(0x0, 0x10);
        let mut object = ObjectBytes::zeroed(0x8);
        routine.assign(&mut object, &[7], 0);
    }
}
