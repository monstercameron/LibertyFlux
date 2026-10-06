//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit routine of the streaming slot-table
//! structures: the eleven entry-table routines verified by lane r-s445,
//! the eleven fixed-table routines verified by lane r-s437, the three
//! small resettable objects of lane r-s436, the three control-block
//! routines of lane r-s441, and the record search and slot liveness of
//! lanes r-s475 and r-s428. `Proven`
//! means the routine is restated on its slots type and the differential
//! test crate ran it against its verified rewrite on the same generated
//! inputs, comparing results and every effect, with a deliberately wrong
//! lift caught alongside. Counts below come from this table.
//!
//! The lane's other structures (the hash tables, the slot arrays, the
//! codec and float tables) have no rows yet: later lanes add one section
//! each.

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
        narrows: &["the 0 answer narrows to unit (every call answers 0)"],
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
            "negative byte offsets are out of domain (no owned entry sits below the base)",
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
    // The fixed-table structures (lane r-s437).
    Row {
        func: "stream_slot_clear_bit3",
        method: "SlotFlags::clear_and_review",
        state: State::Proven,
        narrows: &["instance (3, 4) of the generic clearer; the proof pins the bits"],
    },
    Row {
        func: "stream_slot_clear_bit4",
        method: "SlotFlags::clear_and_review",
        state: State::Proven,
        narrows: &["instance (4, 3) of the generic clearer; the proof pins the bits"],
    },
    Row {
        func: "stream_slot_mark_live",
        method: "SlotFlags::mark_live",
        state: State::Proven,
        narrows: &[
            "the returned slot pointer narrows away (callers own the record; the proof checks the rewrite answers the planted object)",
        ],
    },
    Row {
        func: "stream_find_entry_by_key",
        method: "KeyEntries::find_by_key",
        state: State::Proven,
        narrows: &[
            "the entry address narrows to its index (the proof rebuilds it per case)",
            "the 20 unmodeled entry bytes are filler the proof randomises",
        ],
    },
    Row {
        func: "stream_find_live_entry",
        method: "KeyEntries::find_live",
        state: State::Proven,
        narrows: &[
            "the entry address narrows to its index (the proof rebuilds it per case)",
            "the link address narrows to the owned target state (None is the null link)",
            "the 20 unmodeled entry bytes are filler the proof randomises",
        ],
    },
    Row {
        func: "stream_find_slot_by_state",
        method: "StateSlots::find_first_holding",
        state: State::Proven,
        narrows: &[
            "the slot address narrows to its index (the proof rebuilds it per case)",
            "the proof pins the wanted state to 3",
            "the 108 unmodeled slot bytes are filler the proof randomises",
        ],
    },
    Row {
        func: "stream_entry_lookup_or_alloc",
        method: "AllocTable::lookup_or_alloc",
        state: State::Proven,
        narrows: &[
            "the entry address narrows to its index (the proof rebuilds it per case)",
            "direct indexes past the owned entries answer None; the original reads past its array",
        ],
    },
    Row {
        func: "stream_slot_in_radius",
        method: "GeoSlots::any_in_radius",
        state: State::Proven,
        narrows: &[
            "the 1/0x100 answer narrows to bool",
            "the 31 unmodeled slot bytes are filler the proof randomises",
        ],
    },
    Row {
        func: "stream_slot_adopt",
        method: "AdoptSlot::adopt",
        state: State::Proven,
        narrows: &[
            "the owner field's address narrows to AdoptOutcome::AlreadyAdopted (the proof rebuilds it per case)",
            "the marker's answer travels as an opaque word",
            "the hook's owner-address argument is rebuilt per case; the manager address threaded to the marker is not modeled",
            "the rest of the slot record is filler the proof randomises",
        ],
    },
    Row {
        func: "stream_slot_construct",
        method: "none",
        state: State::Missing,
        narrows: &["one forwarded call plus one address stamp: no behaviour to lift"],
    },
    Row {
        func: "stream_slot_detach",
        method: "none",
        state: State::Missing,
        narrows: &["two forwarded calls whose answer is the result: no behaviour to lift"],
    },
    // The small resettable objects (lane r-s436).
    Row {
        func: "stream_slot_reset",
        method: "ResetSlot::reset",
        state: State::Proven,
        narrows: &[
            "the returned slot pointer narrows away (the proof checks the rewrite answers the planted slot)",
        ],
    },
    Row {
        func: "stream_table_reset",
        method: "LaneTable::reset",
        state: State::Proven,
        narrows: &[
            "the returned table pointer narrows away (the proof checks the rewrite answers the planted table)",
        ],
    },
    Row {
        func: "stream_find_index",
        method: "IdArray::find_from",
        state: State::Proven,
        narrows: &[
            "the key pointer narrows to the wanted value",
            "the -1 answer narrows to None",
            "negative scan indexes and indexes past the owned ids panic; the original reads on with wrapped addresses",
        ],
    },
    // The streaming control block (lane r-s441).
    Row {
        func: "stream_slot_match_f0",
        method: "ControlBlock::match_slot",
        state: State::Proven,
        narrows: &[
            "instance (bank A) of the generic match; the proof pins the bank",
            "the 1/0 answer narrows to bool",
            "the comparison callee becomes the SlotCompare trait (same answers, same call order)",
            "indexes past the owned slots panic; the original reads past its block",
            "the two banks alias past fourteen slots in the original; the proof plants at most ten",
        ],
    },
    Row {
        func: "stream_slot_match_fe",
        method: "ControlBlock::match_slot",
        state: State::Proven,
        narrows: &[
            "instance (bank B) of the generic match; the proof pins the bank",
            "the 1/0 answer narrows to bool",
            "the comparison callee becomes the SlotCompare trait (same answers, same call order)",
            "indexes past the owned slots panic; the original reads past its block",
            "the two banks alias past fourteen slots in the original; the proof plants at most ten",
        ],
    },
    Row {
        func: "stream_slot_reset",
        method: "ControlBlock::reset_slot",
        state: State::Proven,
        narrows: &[
            "the 0 answer narrows to unit",
            "the watcher and release callees become traits (same arguments, same call order)",
            "the two record addresses narrow to block-relative offsets (the proof rebuilds them per case)",
            "indexes past the owned slots panic; the original reads and writes past its block",
        ],
    },
    // Record search and slot liveness (lanes r-s475 and r-s428).
    Row {
        func: "stream_record_find_by_key",
        method: "RecordSet::contains_key",
        state: State::Proven,
        narrows: &[
            "the record stride is proof planting (the scan reads only the keys)",
            "counts past the owned keys panic; the original reads past its array",
        ],
    },
    Row {
        func: "stream_record_lookup",
        method: "RecordSet::position",
        state: State::Proven,
        narrows: &[
            "the record address narrows to its index (the proof rebuilds it per case)",
            "the record stride is proof planting (the scan reads only the keys)",
            "counts past the owned keys panic; the original reads past its array",
        ],
    },
    Row {
        func: "stream_table_get_checked",
        method: "WordTable::get",
        state: State::Proven,
        narrows: &[
            "the null answer narrows to None (a stored zero still answers Some(0))",
            "negative indexes and indexes past the owned words panic; the original reads below or past its array",
        ],
    },
    Row {
        func: "stream_slot_free",
        method: "SlotLiveness::is_free",
        state: State::Proven,
        narrows: &[
            "the stride-0x134 global row narrows to dense bytes",
            "the 1/0 answer narrows to bool",
            "indexes past the owned bytes panic; the original reads past its table",
        ],
    },
    Row {
        func: "stream_slot_init",
        method: "SlotHeader::init",
        state: State::Proven,
        narrows: &["the 0 answer narrows to unit"],
    },
    Row {
        func: "stream_slot_select",
        method: "none",
        state: State::Missing,
        narrows: &["one global word read: no behaviour to lift"],
    },
];
