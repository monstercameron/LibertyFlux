//! Record field access: stores into, and one flag read from, a record that an
//! index selects from a table of record pointers.
//!
//! Several small routines of the original take a stream index and act on the
//! record that the index selects. The record pointer comes from a table, and
//! the routine is guarded by the index's validity byte or by the record pointer
//! itself. The index that means "none" does nothing (or answers zero). A store
//! writes one value into the record at a fixed offset, optionally scaled by an
//! element index. The flag read answers whether one byte of the record is
//! nonzero. [`RecordStore`] and [`RecordFlag`] hold these routines as data;
//! the collaborators are supplied through [`RecordTable`], and the lifted code
//! never dereferences a record pointer itself.
//!
//! Specification of [`RecordStore::store`]:
//!
//! - When the index equals [`RecordStore::null_index`], nothing is written.
//! - When the guard is [`Guard::Validity`], the validity byte for the index is
//!   read, and a zero byte stops the routine. The record pointer is then read.
//! - When the guard is [`Guard::Record`], the record pointer is read first, and
//!   a zero pointer stops the routine.
//! - The store's offset is [`RecordStore::field`] plus the operand, as
//!   [`Element`] gives it: a byte store adds the operand as a byte offset, a
//!   word array store adds four times the operand, and a word store ignores the
//!   operand. The byte store keeps the low byte of the value; a word store keeps
//!   all four bytes.
//!
//! Specification of [`RecordFlag::read`]:
//!
//! - When the index equals [`RecordFlag::null_index`], the result is 0.
//! - When the record pointer for the index is zero, the result is 0.
//! - Otherwise the byte at [`RecordFlag::field`] plus the byte index is read,
//!   and the result is 1 when that byte is nonzero and 0 when it is zero.
//!
//! Offsets are 32-bit wrapping sums, as the original computes them, and nothing
//! here depends on pointer width.

/// The collaborators of the record stores: the record and validity tables and
/// the bytes and words of a record. They are opaque to the lifted routines.
pub trait RecordTable {
    /// The record pointer stored for `index`; zero when no record is present.
    fn record_of(&self, index: u32) -> u32;

    /// The validity byte stored for `index`.
    fn valid(&self, index: u32) -> u8;

    /// The byte at `offset` bytes into `record`.
    fn byte_at(&self, record: u32, offset: u32) -> u8;

    /// Writes `value` at `offset` bytes into `record`.
    fn put_byte(&mut self, record: u32, offset: u32, value: u8);

    /// Writes `value` as four little-endian bytes at `offset` bytes into `record`.
    fn put_word(&mut self, record: u32, offset: u32, value: u32);
}

/// The check that decides whether a store is made.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Guard {
    /// The index's validity byte must be nonzero; the record pointer is not checked.
    Validity,
    /// The index's record pointer must be nonzero; the validity byte is not checked.
    Record,
}

/// How a store's offset and value are formed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Element {
    /// One byte at the field plus the operand; the value's low byte is stored.
    Byte,
    /// One word at the field plus four times the operand; the value is stored.
    WordArray,
    /// One word at the field; the operand is not used.
    Word,
}

/// One guarded store into a record, as data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecordStore {
    /// The index that selects no record.
    pub null_index: u32,
    /// The check that guards the store.
    pub guard: Guard,
    /// Byte offset of the field in the record.
    pub field: u32,
    /// How the offset and the value are formed.
    pub element: Element,
}

impl RecordStore {
    /// Stores `value` into the record that `index` selects, at the offset that
    /// `operand` gives, if the guard and the null check allow it.
    pub fn store(&self, table: &mut impl RecordTable, index: u32, operand: u32, value: u32) {
        if index == self.null_index {
            return;
        }
        let record = match self.guard {
            Guard::Validity => {
                if table.valid(index) == 0 {
                    return;
                }
                table.record_of(index)
            }
            Guard::Record => {
                let record = table.record_of(index);
                if record == 0 {
                    return;
                }
                record
            }
        };
        match self.element {
            Element::Byte => {
                let offset = self.field.wrapping_add(operand);
                table.put_byte(record, offset, value as u8);
            }
            Element::WordArray => {
                let offset = self.field.wrapping_add(operand.wrapping_mul(4));
                table.put_word(record, offset, value);
            }
            Element::Word => table.put_word(record, self.field, value),
        }
    }
}

/// One record flag read, as data: the null index and the byte's field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecordFlag {
    /// The index that selects no record.
    pub null_index: u32,
    /// Byte offset of the flag area in the record.
    pub field: u32,
}

impl RecordFlag {
    /// Answers 1 when the flag byte the operand selects is nonzero in the record
    /// that `index` selects, and 0 otherwise.
    pub fn read(&self, table: &impl RecordTable, index: u32, byte_index: u32) -> u32 {
        if index == self.null_index {
            return 0;
        }
        let record = table.record_of(index);
        if record == 0 {
            return 0;
        }
        let offset = self.field.wrapping_add(byte_index);
        u32::from(table.byte_at(record, offset) != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Element, Guard, RecordFlag, RecordStore, RecordTable};

    /// Three records, each a list of bytes, and one table.
    struct Table {
        records: [Vec<u8>; 3],
        addresses: [u32; 3],
        valid: [u8; 3],
    }

    impl Table {
        fn block(&self, record: u32) -> usize {
            self.addresses
                .iter()
                .position(|&a| a == record)
                .expect("a record pointer of the fixture")
        }
    }

    impl RecordTable for Table {
        fn record_of(&self, index: u32) -> u32 {
            if index < 3 {
                self.addresses[index as usize]
            } else {
                0
            }
        }

        fn valid(&self, index: u32) -> u8 {
            if index < 3 {
                self.valid[index as usize]
            } else {
                0
            }
        }

        fn byte_at(&self, record: u32, offset: u32) -> u8 {
            self.records[self.block(record)][offset as usize]
        }

        fn put_byte(&mut self, record: u32, offset: u32, value: u8) {
            let block = self.block(record);
            self.records[block][offset as usize] = value;
        }

        fn put_word(&mut self, record: u32, offset: u32, value: u32) {
            let block = self.block(record);
            let at = offset as usize;
            self.records[block][at..at + 4].copy_from_slice(&value.to_le_bytes());
        }
    }

    fn fixture(valid: [u8; 3]) -> Table {
        Table {
            records: [vec![0; 16], vec![0; 16], vec![0; 16]],
            addresses: [0x1000, 0x2000, 0x3000],
            valid,
        }
    }

    #[test]
    fn validity_guard_stops_on_a_zero_byte() {
        let store = RecordStore {
            null_index: 9,
            guard: Guard::Validity,
            field: 4,
            element: Element::Word,
        };
        let mut table = fixture([0, 1, 1]);
        store.store(&mut table, 0, 0, 0x0403_0201);
        assert_eq!(table.records[0], vec![0; 16]);
        store.store(&mut table, 1, 0, 0x0403_0201);
        assert_eq!(&table.records[1][4..8], &[1, 2, 3, 4]);
    }

    #[test]
    fn record_guard_stops_on_a_zero_pointer_and_byte_store_takes_the_low_byte() {
        let store = RecordStore {
            null_index: 9,
            guard: Guard::Record,
            field: 2,
            element: Element::Byte,
        };
        let mut table = fixture([1, 1, 1]);
        table.addresses[2] = 0;
        store.store(&mut table, 2, 3, 0x1FF);
        store.store(&mut table, 1, 3, 0x1FF);
        assert_eq!(table.records[1][5], 0xFF);
    }

    #[test]
    fn null_index_writes_nothing_and_flag_answers_zero() {
        let store = RecordStore {
            null_index: 9,
            guard: Guard::Validity,
            field: 0,
            element: Element::Word,
        };
        let mut table = fixture([1, 1, 1]);
        store.store(&mut table, 9, 0, 0xFFFF_FFFF);
        assert!(table.records.iter().all(|r| r.iter().all(|&b| b == 0)));
        let flag = RecordFlag {
            null_index: 9,
            field: 0,
        };
        assert_eq!(flag.read(&table, 9, 0), 0);
    }

    #[test]
    fn word_array_scales_the_operand_by_four() {
        let store = RecordStore {
            null_index: 9,
            guard: Guard::Record,
            field: 4,
            element: Element::WordArray,
        };
        let mut table = fixture([0, 0, 0]);
        store.store(&mut table, 0, 2, 0x0102_0304);
        assert_eq!(&table.records[0][12..16], &[4, 3, 2, 1]);
    }

    #[test]
    fn flag_read_reports_nonzero_bytes_only() {
        let flag = RecordFlag {
            null_index: 9,
            field: 0x0A,
        };
        let mut table = fixture([0, 0, 0]);
        table.records[0][0x0C] = 5;
        assert_eq!(flag.read(&table, 0, 2), 1);
        assert_eq!(flag.read(&table, 0, 3), 0);
        table.addresses[1] = 0;
        assert_eq!(flag.read(&table, 1, 2), 0);
    }
}
