//! Field-to-slot copies: one 32-bit field of an object copied into one word of a table of words.
//!
//! Several small routines of the original copy one word of their object into one global word and return zero.
//! The routines differ only in the field they read and in the global they write. [`FieldSlotCopy`] holds one
//! such routine as data: the field's offset, and the index of the word it writes in the caller's table of
//! words. The caller maps each global to its index; no address appears here.
//!
//! Specification of [`FieldSlotCopy::apply`]:
//!
//! - The word at the field's offset is read from the object, as four bytes least significant first.
//! - That word is stored at index `slot` of the table, and nothing else is written.
//! - The result is always zero.
//! - A field outside the object, or a slot outside the table, is a caller error and panics.

use crate::field_store::ObjectBytes;

/// One field-to-slot copy, as data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldSlotCopy {
    /// Byte offset of the field read from the object.
    pub offset: usize,
    /// Index of the word of the table that receives the field's value.
    pub slot: usize,
}

impl FieldSlotCopy {
    /// Copies the field of `object` into the table and returns zero.
    ///
    /// # Panics
    ///
    /// When the field lies outside the object, or the slot lies outside the table.
    pub fn apply(&self, object: &ObjectBytes, table: &mut [u32]) -> u32 {
        table[self.slot] = object.word(self.offset);
        0
    }
}

#[cfg(test)]
mod tests {
    use super::FieldSlotCopy;
    use crate::field_store::ObjectBytes;

    #[test]
    fn the_field_lands_in_its_slot_and_the_rest_is_untouched() {
        let copy = FieldSlotCopy { offset: 4, slot: 2 };
        let object = ObjectBytes::from_bytes(&[0, 0, 0, 0, 0x78, 0x56, 0x34, 0x12]);
        let mut table = [7u32; 4];
        assert_eq!(copy.apply(&object, &mut table), 0);
        assert_eq!(table, [7, 7, 0x1234_5678, 7]);
    }

    #[test]
    #[should_panic]
    fn a_slot_outside_the_table_panics() {
        let copy = FieldSlotCopy { offset: 0, slot: 4 };
        let object = ObjectBytes::zeroed(4);
        let mut table = [0u32; 4];
        let _ = copy.apply(&object, &mut table);
    }
}
