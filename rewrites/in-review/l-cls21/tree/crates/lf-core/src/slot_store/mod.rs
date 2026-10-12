//! Stores into a slot record that an object selects through a small table.
//!
//! An input object keeps a selector byte. The low three bits of the selector
//! pick one of eight table words. The table word is shifted left by five bits,
//! and the result is an offset from the object: the slot record. The routine
//! then stores the low bits of its argument into one field of that record, at
//! a fixed field offset and with a fixed width.
//!
//! [`SlotStore`] holds one such routine: the field offset and the width in
//! bytes. The object is its bytes, the table is the eight table words the
//! caller has read, and every offset is wrapping 32-bit arithmetic, as in the
//! original. A word is four bytes, little endian. No address appears here.
//!
//! Specification of [`SlotStore::store`]:
//!
//! - The selector is `object[SELECTOR_OFFSET] & SELECTOR_MASK`, and the table
//!   word used is `table[selector]`.
//! - The record offset is `table_word << RECORD_SHIFT`, wrapping, measured
//!   from the start of the object.
//! - The field is written at `record offset + field_offset`, wrapping. The
//!   low `width` bytes of `value` are written in little-endian order. Nothing
//!   else is written.
//!
//! [`SLOT_STORES`] lists the four routines that were lifted from verified
//! rewrites.

/// Byte offset of the selector in the object.
pub const SELECTOR_OFFSET: usize = 0xE7;

/// Mask applied to the selector byte: it picks one of eight table words.
pub const SELECTOR_MASK: u8 = 0x07;

/// Number of entries in the selector table.
pub const TABLE_LEN: usize = 8;

/// Left shift that turns a table word into a record offset.
pub const RECORD_SHIFT: u32 = 5;

/// One stored field of a slot record: its offset within the record and its width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SlotStore {
    /// Byte offset of the field within the slot record.
    pub field_offset: u32,
    /// Width of the stored field in bytes: one or two.
    pub width: usize,
}

impl SlotStore {
    /// A store of `width` bytes at `field_offset` within the record.
    #[must_use]
    pub const fn new(field_offset: u32, width: usize) -> Self {
        Self {
            field_offset,
            width,
        }
    }

    /// Stores the low `width` bytes of `value` into the field of the record the
    /// object's selector names.
    ///
    /// # Panics
    ///
    /// When the field lies outside `object`, or the width is not one or two.
    /// The original would write outside the object; the lift stops instead.
    pub fn store(&self, object: &mut [u8], table: &[u32; TABLE_LEN], value: u32) {
        assert!(
            self.width == 1 || self.width == 2,
            "a field is one or two bytes"
        );
        let selector = usize::from(object[SELECTOR_OFFSET] & SELECTOR_MASK);
        let record = table[selector].wrapping_shl(RECORD_SHIFT);
        let field = record.wrapping_add(self.field_offset) as usize;
        let end = field
            .checked_add(self.width)
            .filter(|&end| end <= object.len())
            .expect("the slot field lies outside the object");
        let bytes = value.to_le_bytes();
        object[field..end].copy_from_slice(&bytes[..self.width]);
    }
}

/// The four stores lifted from verified rewrites: three two-byte fields and one
/// byte field, all in the same kind of record.
pub const SLOT_STORES: [SlotStore; 4] = [
    SlotStore::new(0x5a, 2),
    SlotStore::new(0x58, 2),
    SlotStore::new(0x52, 2),
    SlotStore::new(0x5d, 1),
];

#[cfg(test)]
mod tests {
    use super::{SELECTOR_OFFSET, SLOT_STORES, SlotStore, TABLE_LEN};

    fn object_with_selector(selector: u8) -> Vec<u8> {
        let mut object = vec![0u8; 0x400];
        object[SELECTOR_OFFSET] = selector;
        object
    }

    #[test]
    fn the_selector_picks_the_table_word_and_the_record_follows_it() {
        let mut table = [0u32; TABLE_LEN];
        table[5] = 3; // record at 3 << 5 = 96
        let mut object = object_with_selector(0xFD); // low bits 101 = 5
        SlotStore::new(0x10, 2).store(&mut object, &table, 0xBEEF_1234);
        assert_eq!(&object[96 + 0x10..96 + 0x12], &[0x34, 0x12]);
        assert_eq!(object[96 + 0x12], 0);
    }

    #[test]
    fn a_one_byte_store_writes_only_the_low_byte() {
        let table = [1u32; TABLE_LEN];
        let mut object = object_with_selector(0);
        SlotStore::new(0x20, 1).store(&mut object, &table, 0x1234_5678);
        assert_eq!(object[32 + 0x20], 0x78);
        assert_eq!(object[32 + 0x21], 0);
    }

    #[test]
    fn the_shift_wraps_as_thirty_two_bits() {
        let mut table = [0u32; TABLE_LEN];
        table[0] = 0x0800_0001; // shifted left by five: 0x20 after wrapping
        let mut object = vec![0u8; 0x400];
        SlotStore::new(0, 1).store(&mut object, &table, 9);
        assert_eq!(object[0x20], 9);
    }

    #[test]
    #[should_panic(expected = "outside the object")]
    fn a_field_past_the_object_stops_the_lift() {
        let mut table = [0u32; TABLE_LEN];
        table[0] = 0x100;
        let mut object = vec![0u8; 0x100];
        SlotStore::new(0x5a, 2).store(&mut object, &table, 1);
    }

    #[test]
    fn the_table_has_four_stores_with_the_recorded_fields() {
        assert_eq!(SLOT_STORES.len(), 4);
        assert_eq!(SLOT_STORES[3], SlotStore::new(0x5d, 1));
        assert_eq!(SLOT_STORES.iter().filter(|s| s.width == 2).count(), 3);
    }
}
