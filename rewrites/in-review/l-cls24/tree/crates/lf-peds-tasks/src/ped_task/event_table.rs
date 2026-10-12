//! The task event table: the 32 fixed records the task system scans for
//! deadlines, queued kinds, event keys and owner objects.
//!
//! The original keeps the table as global image data: 32 records at a
//! fixed stride, read and written in place, with the helpers it calls
//! reached by address. This module owns the records as ordinary values in
//! [`EventTable`]. Each word a routine reads or writes is a named field of
//! [`EventRecord`]; the helpers become methods of [`EventCallees`]. Where
//! the original passes a record's address to a helper, the lifted call
//! passes the record's index in the table, and the test crate rebuilds the
//! address from it.
//!
//! The field names follow the routines that use each word. The event word
//! is an active flag in the scans and the lookup key in
//! [`EventTable::find_by_event`]. The kind word is the kind of a queued
//! deadline record in the deadline scans, and the event key in
//! [`EventTable::set_event_flag`]. The object word is a task id in the id
//! scans, and the owner object a record holds in
//! [`EventTable::replace_object`].

/// Number of records in the table.
pub const RECORD_COUNT: usize = 32;

/// The kind of a queued deadline record: the deadline scans select on it,
/// and the non-queued release skips it.
pub const QUEUED_KIND: u32 = 4;

/// A deadline is refreshed only when it lies more than this many ticks
/// beyond the current time (unsigned comparison of the difference).
pub const DELAY_GAP: u32 = 1000;

/// Matching records needed before the lowest-deadline record is released
/// while the selection mode's low byte is zero.
pub const STANDARD_THRESHOLD: u32 = 10;

/// Matching records needed while the selection mode's low byte is nonzero.
pub const REDUCED_THRESHOLD: u32 = 2;

/// Squared distance below which an event is nearby. Equal distances are not.
pub const NEARBY_DISTANCE_SQUARED: f32 = 0.25;

/// One record of the table. Offsets are those of the 32-bit layout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EventRecord {
    /// Word at offset 0x10: the record's event object handle. Nonzero when the
    /// record is active.
    pub event: u32,
    /// Word at offset 0x18: the queued kind, or the event key.
    pub kind: u32,
    /// Word at offset 0x1c: a task id in the id scans, or the owner object
    /// the record holds.
    pub object: u32,
    /// Word at offset 0x24: the deadline in game ticks, compared unsigned.
    pub deadline: u32,
    /// Byte at offset 0xcb: a flag set by [`EventTable::set_event_flag`].
    pub flag: u8,
}

impl EventRecord {
    /// Whether the record is active (its event word is nonzero).
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.event != 0
    }
}

/// The 32 records, owned.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EventTable {
    /// The records, in table order.
    pub records: [EventRecord; RECORD_COUNT],
}

/// The reads of an event object that the nearby check makes. An event is
/// the object word of a record; the reads are the event object's own
/// fields, and the implementation supplies them.
pub trait EventObjects {
    /// The byte at offset 0x1e2 of the event. Its low nibble is the type.
    fn type_byte(&self, event: u32) -> u8;
    /// The word at offset 0x1bc of the event: the vector it is attached to.
    fn vector_id(&self, event: u32) -> u32;
    /// The event's position: three words at offset 0x10, or, when the
    /// pointer at offset 0x20 is nonzero, three words behind that pointer
    /// at offset 0x30. Bits are passed through unchanged.
    fn position(&self, event: u32) -> [f32; 3];
}

/// The helpers the table's routines call. A lifted routine calls these in
/// the same order and with the same arguments as the original; the record
/// arguments are indices into [`EventTable::records`].
pub trait EventCallees {
    /// The delay helper: ticks to add to the current time for a task id.
    fn delay_ticks(&mut self, task_id: u32) -> u32;
    /// The selection-mode helper. Its low byte picks the reduced threshold
    /// when nonzero.
    fn selection_mode(&mut self) -> u32;
    /// The release helper for a queued or active record.
    fn release_record(&mut self, record: usize);
    /// The release helper for the owner object a record holds, called
    /// before the record takes its replacement.
    fn release_object(&mut self, object: u32, record: usize);
    /// The retain helper for the owner object a record now holds.
    fn retain_object(&mut self, object: u32, record: usize);
    /// The proximity helper for a record whose event is nearby. The
    /// 32-bit helper receives the record's address, a fixed option and a
    /// fixed descriptor; the implementation supplies those.
    fn proximity_notify(&mut self, record: usize);
}

impl EventTable {
    /// Refreshes the deadline of every queued record of task `task_id`
    /// that is active and whose deadline lies more than [`DELAY_GAP`]
    /// ticks beyond `current_time`. Its new deadline is the current time
    /// plus the delay the helper returns.
    pub fn refresh_matching_deadlines<C: EventCallees>(
        &mut self,
        task_id: u32,
        current_time: u32,
        callees: &mut C,
    ) {
        for record in &mut self.records {
            if !record.is_active() || record.kind != QUEUED_KIND || record.object != task_id {
                continue;
            }
            if record.deadline.wrapping_sub(current_time) > DELAY_GAP {
                let delay = callees.delay_ticks(task_id);
                record.deadline = current_time.wrapping_add(delay);
            }
        }
    }

    /// Releases the queued record of task `task_id` with the lowest
    /// deadline, once enough queued records of that task match. Counting
    /// stops at the first lowest deadline on a tie, and the threshold is
    /// [`STANDARD_THRESHOLD`] or [`REDUCED_THRESHOLD`] by the mode helper.
    /// The mode helper is called on every run, after the scan.
    pub fn release_lowest_deadline_record<C: EventCallees>(
        &mut self,
        task_id: u32,
        callees: &mut C,
    ) {
        let mut matches = 0_u32;
        let mut lowest: Option<(usize, u32)> = None;
        for (index, record) in self.records.iter().enumerate() {
            if !record.is_active() || record.kind != QUEUED_KIND || record.object != task_id {
                continue;
            }
            matches = matches.wrapping_add(1);
            // Replace only on a strictly lower deadline, so the first record wins a tie.
            if lowest.is_none_or(|(_, best)| record.deadline < best) {
                lowest = Some((index, record.deadline));
            }
        }

        let mode = callees.selection_mode();
        let threshold = if (mode & 0xFF) != 0 {
            REDUCED_THRESHOLD
        } else {
            STANDARD_THRESHOLD
        };
        if let Some((index, _)) = lowest {
            if matches >= threshold {
                callees.release_record(index);
            }
        }
    }

    /// The index of the first record whose event word equals `key`. A zero
    /// key matches nothing.
    #[must_use]
    pub fn find_by_event(&self, event: u32) -> Option<usize> {
        if event == 0 {
            return None;
        }
        self.records.iter().position(|record| record.event == event)
    }

    /// Whether record `index` exists and its object word equals `object`.
    /// An index past the table never matches.
    #[must_use]
    pub fn holds_object(&self, index: u32, object: u32) -> bool {
        usize::try_from(index)
            .ok()
            .and_then(|index| self.records.get(index))
            .is_some_and(|record| record.object == object)
    }

    /// Stores the low byte of `flag` in the flag of every active record
    /// whose kind equals `event_key`.
    pub fn set_event_flag(&mut self, event_key: u32, flag: u32) {
        // The low byte is the stored value: the original writes one byte.
        let stored = (flag & 0xFF) as u8;
        for record in &mut self.records {
            if record.is_active() && record.kind == event_key {
                record.flag = stored;
            }
        }
    }

    /// Calls the release helper for each record that `select` accepts, in
    /// table order. The table itself is not changed.
    pub fn release_where<C, P>(&self, select: P, callees: &mut C)
    where
        C: EventCallees,
        P: Fn(&EventRecord) -> bool,
    {
        for (index, record) in self.records.iter().enumerate() {
            if select(record) {
                callees.release_record(index);
            }
        }
    }

    /// Releases every active record.
    pub fn release_active<C: EventCallees>(&self, callees: &mut C) {
        self.release_where(EventRecord::is_active, callees);
    }

    /// Releases every active record whose kind is not the queued kind.
    pub fn release_nonqueued<C: EventCallees>(&self, callees: &mut C) {
        self.release_where(
            |record| record.is_active() && record.kind != QUEUED_KIND,
            callees,
        );
    }

    /// Replaces the owner object of record `index`. An index past the table
    /// is ignored, and so is a replacement equal to the current object. A
    /// nonzero current object is released before the store, and a nonzero
    /// replacement is retained after it.
    pub fn replace_object<C: EventCallees>(
        &mut self,
        index: u32,
        replacement: u32,
        callees: &mut C,
    ) {
        let Some(index) = usize::try_from(index).ok().filter(|&i| i < RECORD_COUNT) else {
            return;
        };
        let current = self.records[index].object;
        if current == replacement {
            return;
        }
        if current != 0 {
            callees.release_object(current, index);
        }
        self.records[index].object = replacement;
        if replacement != 0 {
            callees.retain_object(replacement, index);
        }
    }

    /// Calls the proximity helper for each record whose object is an event
    /// attached to the target vector (`target_id`) and lies strictly within
    /// the nearby distance of `target`. An event attached with a type below
    /// two counts as attached to vector zero. The squared distance is
    /// computed in single precision: the differences are formed and squared
    /// in y, x, z order, and the squares are summed in the same order.
    pub fn check_nearby_event<O, C>(
        &self,
        target_id: u32,
        target: [f32; 3],
        objects: &O,
        callees: &mut C,
    ) where
        O: EventObjects,
        C: EventCallees,
    {
        for (index, record) in self.records.iter().enumerate() {
            let event = record.event;
            if event == 0 {
                continue;
            }
            let event_type = objects.type_byte(event) & 0x0F;
            let vector_id = if event_type >= 2 {
                objects.vector_id(event)
            } else {
                0
            };
            if vector_id != target_id {
                continue;
            }

            let [event_x, event_y, event_z] = objects.position(event);
            let [target_x, target_y, target_z] = target;
            let y_delta = event_y - target_y;
            let x_delta = event_x - target_x;
            let z_delta = event_z - target_z;
            let y_squared = y_delta * y_delta;
            let x_squared = x_delta * x_delta;
            let z_squared = z_delta * z_delta;
            let distance_squared = (y_squared + x_squared) + z_squared;
            if distance_squared < NEARBY_DISTANCE_SQUARED {
                callees.proximity_notify(index);
            }
        }
    }
}
