//! The record query of the layout frame.
//!
//! The frame asks itself for a container of records. The container holds a
//! count and an array of records, and each record carries a flag byte. The
//! query answers whether, among the records, both of two flag bits are set,
//! where a record that carries the first bit counts for the first bit only.
//! The answer keeps the container's high 24 bits and puts the boolean in the
//! low byte, as the original's answer word does.
//!
//! The container and records are memory the frame does not own, so the
//! method takes a [`RecordHost`] that answers for them.

use super::frame::UILayoutFrame;

/// The first flag bit a record may carry.
const FIRST_FLAG: u8 = 0x02;

/// The second flag bit a record may carry, counted only when the first is not
/// set on that record.
const SECOND_FLAG: u8 = 0x08;

/// Both flags, which the query answers true for.
const COMPLETE_FLAGS: u8 = FIRST_FLAG | SECOND_FLAG;

/// The calls the record query makes outside the frame's bytes.
pub trait RecordHost {
    /// The frame's own record query: the container it answers with. The
    /// answer is opaque to the lift except for its high 24 bits, which the
    /// query's answer keeps.
    fn query_records(&mut self, frame: &mut UILayoutFrame) -> u32;

    /// The number of records in the container.
    fn record_count(&mut self, container: u32) -> u16;

    /// The flag byte of the record at `index` in the container. The index is
    /// below the count the host answered for the container.
    fn record_flags(&mut self, container: u32, index: usize) -> u8;
}

impl UILayoutFrame {
    /// Asks the frame for its record container and answers whether both flag
    /// bits are set among its records. Zero and one record answer false
    /// without reading any record. The answer is the container's high 24 bits
    /// with the boolean in the low byte.
    pub fn two_flags_set_among_records<H: RecordHost>(&mut self, host: &mut H) -> u32 {
        let container = host.query_records(self);
        let count = host.record_count(container);
        let mut observed = 0u8;
        if count > 1 {
            for index in 0..usize::from(count) {
                let flags = host.record_flags(container, index);
                if flags & FIRST_FLAG != 0 {
                    observed |= FIRST_FLAG;
                } else if flags & SECOND_FLAG != 0 {
                    observed |= SECOND_FLAG;
                }
            }
        }
        (container & 0xFFFF_FF00) | u32::from(observed == COMPLETE_FLAGS)
    }
}
