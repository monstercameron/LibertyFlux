//! The streaming slot table: entries that record what is loaded.
//!
//! Lifted from the verified rewrites of the free streaming routines (no
//! class): the entry cluster (init, active tests, block count, data
//! address, location, slot index, mask test, kind flag, flag set, range
//! check), the slot flag word (two clearer instances, one marker), the
//! fixed tables (keyed entries, state slots, the alloc table, geometric
//! slots) and the slot adoption record. The 32-bit form holds entry
//! arrays behind table objects; the lift owns the entries as vectors.
//!
//! No addresses, no numbered slots, no global state, no pointer-width
//! dependence, no `unsafe`.

mod adopt;
mod entry;
mod fixed;
mod flags;
mod registry;
mod table;

pub use adopt::AdoptHook;
pub use adopt::AdoptOutcome;
pub use adopt::AdoptSlot;
pub use adopt::MarkSlot;
pub use entry::DataAddress;
pub use entry::DataSlots;
pub use entry::KindBytes;
pub use entry::KindSlots;
pub use entry::StreamEntry;
pub use fixed::ALLOC_ENTRY_LEN;
pub use fixed::AllocEntry;
pub use fixed::AllocTable;
pub use fixed::GEO_SLOT_LEN;
pub use fixed::GeoSlot;
pub use fixed::GeoSlots;
pub use fixed::KEY_ENTRY_LEN;
pub use fixed::KeyEntries;
pub use fixed::KeyEntry;
pub use fixed::STATE_SLOT_LEN;
pub use fixed::StateSlots;
pub use flags::SlotFlags;
pub use registry::Row;
pub use registry::State;
pub use registry::ROWS;
pub use table::SlotTable;
