//! The typed-record scan: one pass over the task system's 32-entry record
//! table, collecting the records of three tags whose fetched point lies
//! strictly inside a radius around an anchor.
//!
//! The owning type is [`RecordTable`]: the thirty-two records, each held as
//! its type tag (the record's first word; the rest of the record is not
//! read here). A record whose tag is one of the three scanned tags is handed
//! to the fetch callee together with the scratch slot that tag selects; the
//! callee answers four single-precision values. A point is kept when its
//! squared distance to the anchor is strictly below the squared radius, and
//! each kept point becomes a [`ScanEntry`]: the four fetched values, then
//! four constants.
//!
//! The fetch callee and the anchor, which the original reads through the
//! object's pointer for each record, travel as [`FetchCallee`]. The scan
//! stops after the entry count reaches the limit, checked after each entry
//! is kept, or at the end of the table.

/// Records in the table.
pub const TABLE_SLOTS: usize = 32;

/// Type tag of records scanned into the first scratch slot.
pub const TAG_SLOT_FIRST: u32 = 0x11;
/// Type tag of records scanned into the second scratch slot.
pub const TAG_SLOT_SECOND: u32 = 0x12;
/// Type tag of records scanned into the third scratch slot.
pub const TAG_SLOT_THIRD: u32 = 0x1B;

/// The four constants each entry carries after its fetched values: 4.0,
/// 16.0, 500.0 and 1000.0.
pub const ENTRY_CONSTANTS: [f32; 4] = [4.0, 16.0, 500.0, 1000.0];

/// The scratch buffer a fetched record's values are written to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScratchSlot {
    /// The buffer of the first tag.
    First,
    /// The buffer of the second tag.
    Second,
    /// The buffer of the third tag.
    Third,
}

/// One record of the table, as far as the scan reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TableRecord {
    /// The record's type tag, its first word.
    pub tag: u32,
}

/// One kept point: the four fetched values and the four entry constants.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScanEntry {
    /// The fetched values, in order.
    pub points: [f32; 4],
    /// The constants that follow the values in the output.
    pub constants: [f32; 4],
}

/// The collaborators of the scan: the fetch callee, which takes a record
/// (by its index in the table) and the scratch slot its tag selects, and the
/// anchor point the distance is measured from.
pub trait FetchCallee {
    /// The anchor point. The original reads it through the object's pointer
    /// for each record it considers.
    fn anchor(&self) -> Anchor;
    /// Fetches the four values of a record. The callee writes them into the
    /// scratch slot and answers a pointer to them; the lift takes the values.
    fn fetch(&mut self, record: usize, slot: ScratchSlot) -> [f32; 4];
}

/// The anchor point: three single-precision components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Anchor {
    /// The first component.
    pub x: f32,
    /// The second component.
    pub y: f32,
    /// The third component.
    pub z: f32,
}

/// The record table, owned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordTable {
    /// The records, in table order.
    pub records: [TableRecord; TABLE_SLOTS],
}

impl Default for RecordTable {
    fn default() -> Self {
        Self {
            records: [TableRecord::default(); TABLE_SLOTS],
        }
    }
}

/// The scratch slot a tag selects, or none when the tag is not scanned.
#[must_use]
pub fn slot_for_tag(tag: u32) -> Option<ScratchSlot> {
    match tag {
        TAG_SLOT_FIRST => Some(ScratchSlot::First),
        TAG_SLOT_SECOND => Some(ScratchSlot::Second),
        TAG_SLOT_THIRD => Some(ScratchSlot::Third),
        _ => None,
    }
}

impl RecordTable {
    /// Scans the table and returns the kept points in table order.
    ///
    /// The distance is squared and compared with the squared radius by a
    /// strict greater-than, so a not-a-number on either side keeps nothing.
    /// The limit is checked after each kept point: the scan stops once the
    /// count is at least `max` (a signed comparison).
    pub fn collect_near(
        &self,
        radius: f32,
        max: i32,
        callee: &mut impl FetchCallee,
    ) -> Vec<ScanEntry> {
        let limit = radius * radius;
        let mut kept = Vec::new();
        for (index, record) in self.records.iter().enumerate() {
            let Some(slot) = slot_for_tag(record.tag) else {
                continue;
            };
            let points = callee.fetch(index, slot);
            let anchor = callee.anchor();
            let dx = points[0] - anchor.x;
            let dy = points[1] - anchor.y;
            let dz = points[2] - anchor.z;
            let dist = ((dy * dy) + (dx * dx)) + (dz * dz);
            if limit > dist {
                kept.push(ScanEntry {
                    points,
                    constants: ENTRY_CONSTANTS,
                });
                if kept.len() as i32 >= max {
                    break;
                }
            }
        }
        kept
    }
}
