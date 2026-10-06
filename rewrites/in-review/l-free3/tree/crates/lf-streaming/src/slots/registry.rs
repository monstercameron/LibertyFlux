//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit routine of the streaming entry-table
//! structure: the eleven entry routines verified by lane r-s445. `Proven`
//! means the routine is restated on [`StreamEntry`](crate::slots::StreamEntry)
//! or [`SlotTable`](crate::slots::SlotTable) and the differential test
//! crate ran it against its verified rewrite on the same generated
//! inputs, comparing results and every effect, with a deliberately wrong
//! lift caught alongside. Counts below come from this table.
//!
//! The lane's other structures (the fixed 256-entry/64-slot tables, the
//! hash tables, the slot arrays, the codec and float tables) have no rows
//! yet: later lanes add one section each.

/// Lift state of one verified routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its slots type and proven against its rewrite.
    Proven,
    /// Restated but not proven (no row should stay here: everything
    /// lifted in this module is proven).
    Lifted,
    /// Not lifted, for the stated reason.
    Missing,
}

/// One verified routine's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Naming-lane name of the verified routine, e.g. `"stream_entry_init"`.
    pub func: &'static str,
    /// Lifted method, e.g. `"StreamEntry::empty"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the routine is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified routine of the streaming entry-table structure.
pub const ROWS: &[Row] = &[
    Row {
        func: "stream_entry_init",
        method: "StreamEntry::empty",
        state: State::Proven,
        narrows: &[
            "the 0 answer narrows to unit (every call answers 0)",
        ],
    },
    Row {
        func: "stream_entry_is_active",
        method: "StreamEntry::is_active",
        state: State::Proven,
        narrows: &[
            "the 1/0 answer narrows to bool",
            "the upper bytes of the answer register are the caller's residue; only the low byte is compared",
        ],
    },
    Row {
        func: "stream_entry_block_count",
        method: "StreamEntry::block_count",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        func: "stream_entry_data_address",
        method: "StreamEntry::data_address",
        state: State::Proven,
        narrows: &[
            "the global slot region narrows to 256 owned slot values (stride-160 layout is proof planting)",
        ],
    },
    Row {
        func: "stream_entry_get_location",
        method: "StreamEntry::locate",
        state: State::Proven,
        narrows: &[
            "the 1/0 answer and the two out words narrow to Option<(u32, u32)>",
            "the address callee becomes the DataAddress trait (same answers, same call order)",
        ],
    },
    Row {
        func: "stream_entry_slot_index",
        method: "SlotTable::relative_slot",
        state: State::Proven,
        narrows: &[
            "the entry address narrows to its index (byte offset idx * 24; the magic divide runs on it exactly)",
            "the kind table's stride-100 layout narrows to 256 owned base slots",
            "indexes past the owned entries panic; the original reads past its array",
            "the planted table base is pinned equal to the entry-array base or above it by whole entries in tested cases",
        ],
    },
    Row {
        func: "stream_slot_in_range",
        method: "SlotTable::is_slot_usable",
        state: State::Proven,
        narrows: &[
            "the null base narrows to an empty entry vector",
            "the 1/0 answer narrows to bool",
            "the upper bytes of the answer register are the caller's residue; only the low byte is compared",
        ],
    },
    Row {
        func: "stream_entry_test_mask",
        method: "SlotTable::test_mask",
        state: State::Proven,
        narrows: &[
            "the 1/0 answer narrows to bool",
            "indexes past the owned entries panic; the original reads past its array",
        ],
    },
    Row {
        func: "stream_table_entry_is_active",
        method: "SlotTable::entry_is_active",
        state: State::Proven,
        narrows: &[
            "the null base narrows to an empty entry vector",
            "the 1/0 answer narrows to bool",
            "the upper bytes of the answer register are the caller's residue; only the low byte is compared",
            "indexes past a non-empty table panic; the original reads past its array",
        ],
    },
    Row {
        func: "stream_entry_kind_flag",
        method: "SlotTable::kind_flag",
        state: State::Proven,
        narrows: &[
            "the global kind region narrows to owned flag bytes (stride-160 layout is proof planting)",
            "indexes past the owned entries panic; the original reads past its array",
        ],
    },
    Row {
        func: "stream_entry_set_flag_8000",
        method: "SlotTable::set_flag_bit15",
        state: State::Proven,
        narrows: &[
            "the returned table base narrows away (callers own the table; the proof checks the rewrite returns the planted base)",
            "indexes past the owned entries panic; the original writes past its array",
        ],
    },
];
