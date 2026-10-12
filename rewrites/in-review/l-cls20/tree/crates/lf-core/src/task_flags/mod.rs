//! Reading one flag bit of the object a task id selects.
//!
//! A task record keeps a signed 16-bit task id at a fixed byte offset. The id
//! selects an object through a pointer table, and that object keeps a flags
//! word at a fixed byte offset. Each routine returns one bit of that flags
//! word, as zero or one. The routines differ only in the bit they read.
//!
//! [`TaskFlagBit`] holds one such routine: the bit index. The record is its
//! bytes. The table is not read here: the caller supplies, for each id, the
//! bytes of the object the table names for that id. That keeps every address
//! and the table layout out of this module. What is lifted is the decoding of
//! the id, the flags-word read at its offset, and the bit extraction.
//!
//! Specification of [`TaskFlagBit::bit_of_entry`]:
//!
//! - The id is the signed 16-bit value at [`ID_OFFSET`] in the record, little
//!   endian. It is sign-extended, so an id of minus one selects the entry for
//!   minus one.
//! - The object for that id is asked for through the caller's `object_of`.
//! - The flags word is the 32-bit word at [`FLAGS_OFFSET`] of that object.
//! - The result is bit `bit` of the flags word, as zero or one.
//! - Nothing is written.
//!
//! [`TASK_FLAG_BITS`] lists the bits of the routines lifted from verified
//! rewrites.

use crate::object_rows::read_word;

/// Byte offset of the signed task id in the record.
pub const ID_OFFSET: usize = 0x2E;

/// Byte offset of the flags word in the object the table names.
pub const FLAGS_OFFSET: usize = 0x94;

/// One routine that returns a single bit of an object's flags word.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaskFlagBit {
    /// Index of the bit that is returned, zero for the least significant.
    pub bit: u32,
}

impl TaskFlagBit {
    /// A routine that returns bit `bit` of the flags word.
    #[must_use]
    pub const fn new(bit: u32) -> Self {
        Self { bit }
    }

    /// The bit of the selected object's flags word, as zero or one.
    ///
    /// `object_of` gives the bytes of the object the table names for one id.
    ///
    /// # Panics
    ///
    /// When the id field lies outside `record`, or the object `object_of`
    /// returns does not hold the flags word.
    pub fn bit_of_entry<'a, F>(&self, record: &[u8], object_of: F) -> u32
    where
        F: Fn(i16) -> &'a [u8],
    {
        let id = read_id(record);
        let object = object_of(id);
        let flags = read_word(object, FLAGS_OFFSET);
        (flags >> self.bit) & 1
    }
}

/// The signed 16-bit task id at [`ID_OFFSET`] of a record, sign-extended.
///
/// # Panics
///
/// When the id field lies outside `record`.
#[must_use]
pub fn read_id(record: &[u8]) -> i16 {
    let end = ID_OFFSET + 2;
    assert!(end <= record.len(), "the task id lies outside the record");
    i16::from_le_bytes([record[ID_OFFSET], record[ID_OFFSET + 1]])
}

/// The bits of the four routines lifted from verified rewrites.
pub const TASK_FLAG_BITS: [TaskFlagBit; 4] = [
    TaskFlagBit::new(26),
    TaskFlagBit::new(14),
    TaskFlagBit::new(15),
    TaskFlagBit::new(25),
];

#[cfg(test)]
mod tests {
    use super::{FLAGS_OFFSET, ID_OFFSET, TASK_FLAG_BITS, TaskFlagBit, read_id};

    fn record_with_id(id: i16) -> Vec<u8> {
        let mut record = vec![0u8; 0x40];
        record[ID_OFFSET..ID_OFFSET + 2].copy_from_slice(&id.to_le_bytes());
        record
    }

    fn object_with_flags(flags: u32) -> Vec<u8> {
        let mut object = vec![0u8; FLAGS_OFFSET + 4];
        object[FLAGS_OFFSET..].copy_from_slice(&flags.to_le_bytes());
        object
    }

    #[test]
    fn the_id_is_sign_extended() {
        assert_eq!(read_id(&record_with_id(-1)), -1);
        assert_eq!(read_id(&record_with_id(i16::MIN)), i16::MIN);
        assert_eq!(read_id(&record_with_id(300)), 300);
    }

    #[test]
    fn the_bit_of_the_selected_object_is_returned() {
        let flags = 1u32 << 14;
        let object = object_with_flags(flags);
        let record = record_with_id(7);
        let got = TaskFlagBit::new(14).bit_of_entry(&record, |id| {
            assert_eq!(id, 7);
            object.as_slice()
        });
        assert_eq!(got, 1);
        assert_eq!(
            TaskFlagBit::new(13).bit_of_entry(&record, |_| object.as_slice()),
            0
        );
    }

    #[test]
    fn a_negative_id_reaches_the_caller_as_a_negative_id() {
        let object = object_with_flags(u32::MAX);
        let record = record_with_id(-2);
        let got = TaskFlagBit::new(31).bit_of_entry(&record, |id| {
            assert_eq!(id, -2);
            object.as_slice()
        });
        assert_eq!(got, 1);
    }

    #[test]
    #[should_panic(expected = "outside the object")]
    fn an_object_without_the_flags_word_stops_the_lift() {
        let object = vec![0u8; 0x40];
        let record = record_with_id(0);
        let _ = TaskFlagBit::new(0).bit_of_entry(&record, |_| object.as_slice());
    }

    #[test]
    fn the_table_has_four_bits_of_the_recorded_routines() {
        let bits: Vec<u32> = TASK_FLAG_BITS.iter().map(|b| b.bit).collect();
        assert_eq!(bits, vec![26, 14, 15, 25]);
    }
}
