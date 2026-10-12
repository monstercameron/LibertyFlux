//! Record word reads: one word of the record that an index selects from a
//! table of record pointers, read only when the index is valid and the record
//! is present.
//!
//! Several small routines of the original keep a table of record pointers and
//! a table of validity bytes, both indexed by one small integer. A routine is
//! given the index. It answers a fixed value when the index is the table's
//! "no index" value, and a second fixed value when the record pointer is
//! zero or the validity byte is zero. Otherwise it reads one word of the
//! record at a fixed byte offset and returns it. The routines differ in the
//! offset and in the two fixed answers. [`RecordWordRead`] holds one such
//! routine as data, and [`RecordWordRead::read`] is the routine. The
//! collaborators are the two tables, which the caller supplies through
//! [`RecordSlots`]; the lifted code never dereferences a record pointer itself.
//!
//! Specification of [`RecordWordRead::read`]:
//!
//! - When the index equals [`RecordWordRead::null_index`], the result is
//!   [`RecordWordRead::null_result`]. The tables are not read.
//! - Otherwise the record pointer for the index is read. When it is zero, the
//!   result is [`RecordWordRead::miss_result`] and the validity byte is not
//!   read.
//! - Otherwise the validity byte for the index is read. When it is zero, the
//!   result is [`RecordWordRead::miss_result`].
//! - Otherwise the result is the 32-bit word at [`RecordWordRead::field`] bytes
//!   into the record, as [`RecordSlots::word`] gives it.
//! - Nothing is written.
//!
//! Index arithmetic is the caller's concern: the table entry for an index is
//! the one [`RecordSlots::record_of`] and [`RecordSlots::valid`] give, and the
//! lifted code compares indices as 32-bit values.

/// The collaborators of a record word read: the record table, the validity
/// table and the record words. They are opaque to the lifted routine.
pub trait RecordSlots {
    /// The record pointer stored for `index`; zero when no record is present.
    fn record_of(&self, index: u32) -> u32;

    /// The validity byte stored for `index`; zero when the record is not valid.
    fn valid(&self, index: u32) -> u8;

    /// The 32-bit word at `offset` bytes into the record `record`.
    fn word(&self, record: u32, offset: u32) -> u32;
}

/// One record word read, as data: the index that means "none", the two fixed
/// answers and the field offset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecordWordRead {
    /// The index that selects no record.
    pub null_index: u32,
    /// The answer for the null index.
    pub null_result: u32,
    /// The answer when the record is absent or not valid.
    pub miss_result: u32,
    /// Byte offset of the word read from the record.
    pub field: u32,
}

impl RecordWordRead {
    /// Reads the word the index selects, or the fixed answer that applies, as
    /// the specification in the module documentation says.
    pub fn read(&self, slots: &impl RecordSlots, index: u32) -> u32 {
        if index == self.null_index {
            return self.null_result;
        }
        let record = slots.record_of(index);
        if record == 0 {
            return self.miss_result;
        }
        if slots.valid(index) == 0 {
            return self.miss_result;
        }
        slots.word(record, self.field)
    }
}

#[cfg(test)]
mod tests {
    use super::{RecordSlots, RecordWordRead};

    /// Two records of eight words each, one valid and one not, and a table.
    struct Fixture;

    impl RecordSlots for Fixture {
        fn record_of(&self, index: u32) -> u32 {
            match index {
                0 => 0x100,
                1 => 0x200,
                2 => 0,
                _ => 0x300,
            }
        }

        fn valid(&self, index: u32) -> u8 {
            u8::from(index != 1)
        }

        fn word(&self, record: u32, offset: u32) -> u32 {
            record.wrapping_add(offset)
        }
    }

    const READ: RecordWordRead = RecordWordRead {
        null_index: 0xFFFF_FFA6,
        null_result: 0xFFFF_FFA4,
        miss_result: 0,
        field: 0x10,
    };

    #[test]
    fn valid_record_gives_its_word() {
        assert_eq!(READ.read(&Fixture, 0), 0x110);
    }

    #[test]
    fn null_index_gives_the_null_answer() {
        assert_eq!(READ.read(&Fixture, 0xFFFF_FFA6), 0xFFFF_FFA4);
    }

    #[test]
    fn absent_and_invalid_records_give_the_miss_answer() {
        assert_eq!(READ.read(&Fixture, 2), 0);
        assert_eq!(READ.read(&Fixture, 1), 0);
    }
}
