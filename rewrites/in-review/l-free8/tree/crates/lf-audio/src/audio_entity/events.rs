//! The audio entity's event table.
//!
//! The entity holds a table of event rows (reached through the word at
//! `this+0xE8` in the 32-bit form). A key selects a row; each row keeps
//! up to twelve records plus a count byte. Appending past a full row is
//! a silent no-op. Row addresses narrow to row indexes: the key must
//! select an owned row.

/// Maximum records per row; the count byte saturates here.
pub const MAX_RECORDS: usize = 12;

/// One event record: a parameter word, two flag bytes, six payload
/// bytes and a 24-byte blob. In the 32-bit row each record is a
/// 40-byte slot holding these fields at fixed offsets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventRecord {
    /// Parameter word of the record.
    pub param: u32,
    /// First flag byte.
    pub flag0: u8,
    /// Second flag byte.
    pub flag1: u8,
    /// Six payload bytes copied from the caller's short buffer.
    pub small: [u8; 6],
    /// 24 payload bytes copied from the caller's long buffer.
    pub blob: [u8; 24],
}

/// One event row: the records appended so far, at most [`MAX_RECORDS`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EventRow {
    /// Records in append order.
    pub records: Vec<EventRecord>,
}

/// The entity's event table: one row per key.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EventTable {
    /// Rows by key; key `k` appends to `rows[k]`.
    pub rows: Vec<EventRow>,
}

impl EventTable {
    /// A table of `n` empty rows.
    #[must_use]
    pub fn with_rows(n: usize) -> Self {
        Self {
            rows: (0..n).map(|_| EventRow::default()).collect(),
        }
    }

    /// Appends `record` to row `key`.
    ///
    /// Returns false, appending nothing, once the row holds
    /// [`MAX_RECORDS`] records; the 32-bit form also answers 0 on both
    /// paths, so the answer narrows to "was it appended".
    ///
    /// # Panics
    ///
    /// When `key` does not select an owned row. The 32-bit form would
    /// write at a computed address instead; out-of-table keys are out
    /// of the lift's domain.
    pub fn append(&mut self, key: u32, record: EventRecord) -> bool {
        let row = self
            .rows
            .get_mut(key as usize)
            .expect("event-table key selects an owned row");
        if row.records.len() >= MAX_RECORDS {
            return false;
        }
        row.records.push(record);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(param: u32) -> EventRecord {
        EventRecord {
            param,
            flag0: 1,
            flag1: 2,
            small: [3; 6],
            blob: [4; 24],
        }
    }

    #[test]
    fn append_fills_row_then_refuses() {
        let mut table = EventTable::with_rows(1);
        for i in 0..MAX_RECORDS {
            assert!(table.append(0, sample(i as u32)), "append {i}");
        }
        assert_eq!(table.rows[0].records.len(), MAX_RECORDS);
        assert!(!table.append(0, sample(99)));
        assert_eq!(table.rows[0].records.len(), MAX_RECORDS);
        // Order and content survive.
        assert_eq!(table.rows[0].records[0].param, 0);
        assert_eq!(table.rows[0].records[11].param, 11);
    }

    #[test]
    fn keys_select_rows_independently() {
        let mut table = EventTable::with_rows(3);
        assert!(table.append(2, sample(7)));
        assert!(table.rows[0].records.is_empty());
        assert!(table.rows[1].records.is_empty());
        assert_eq!(table.rows[2].records.len(), 1);
    }

    #[test]
    #[should_panic(expected = "owned row")]
    fn key_past_owned_rows_panics() {
        let mut table = EventTable::with_rows(1);
        let _ = table.append(1, sample(0));
    }

    #[test]
    fn empty_table_has_no_rows() {
        let table = EventTable::with_rows(0);
        assert!(table.rows.is_empty());
    }
}
