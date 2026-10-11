//! The task id lookup: a keyed table is asked first, then a fallback, and
//! the id of the record it finds is the answer, with a default for a
//! record that carries no id.
//!
//! [`TaskKey::id_or_default`] is the one verified routine of this family.
//! It reads the key the task owner holds, asks the primary table, asks the
//! fallback when the primary has no record, takes the id word of the record
//! it ends with, and gives the default id when that word is the all-ones
//! sentinel.

#![forbid(unsafe_code)]

/// The id returned when the record's id word is the sentinel.
pub const DEFAULT_ID: u32 = 0x31;
/// The sentinel the record's id word holds when it carries no id.
pub const NO_ID: u32 = 0xFFFF_FFFF;

/// The lookups behind [`TaskKey::id_or_default`]: two keyed tables and the
/// id word of a record.
///
/// A record is an opaque handle; the routine never reads one except
/// through [`TaskIdTable::record_id`].
pub trait TaskIdTable {
    /// Asks the primary table for the record of `key`; zero means no record.
    fn primary(&mut self, key: u32) -> u32;
    /// Asks the fallback table for the record of `key`.
    fn fallback(&mut self, key: u32) -> u32;
    /// The id word of a record (its word four bytes in).
    fn record_id(&mut self, record: u32) -> u32;
}

/// The task's key: the value the lookups are keyed by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskKey {
    /// The key the owner holds.
    pub key: u32,
}

impl TaskKey {
    /// The task's id: the id of the record the lookups end on, or
    /// [`DEFAULT_ID`] when that record's id word is [`NO_ID`].
    ///
    /// The fallback is asked only when the primary answer is zero. The id
    /// word is read from the final record whichever table gave it.
    pub fn id_or_default<T: TaskIdTable>(&self, table: &mut T) -> u32 {
        let mut record = table.primary(self.key);
        if record == 0 {
            record = table.fallback(self.key);
        }
        let id = table.record_id(record);
        if id == NO_ID { DEFAULT_ID } else { id }
    }
}
