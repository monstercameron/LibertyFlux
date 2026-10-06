//! The streaming slot table: entries that record what is loaded.
//!
//! Lifted from the verified rewrites of the free streaming routines (no
//! class): the entry cluster (init, active tests, block count, data
//! address, location, slot index, mask test, kind flag, flag set, range
//! check). The 32-bit form is a table object holding an entry-array base
//! at `+0x00` and a capacity at `+0x04`, over 24-byte entries; the lift
//! owns the entries as a vector and the capacity as a signed word.
//!
//! No addresses, no numbered slots, no global state, no pointer-width
//! dependence, no `unsafe`.

mod entry;
mod registry;
mod table;

pub use entry::DataAddress;
pub use entry::DataSlots;
pub use entry::KindBytes;
pub use entry::KindSlots;
pub use entry::StreamEntry;
pub use registry::ROWS;
pub use registry::Row;
pub use registry::State;
pub use table::SlotTable;
