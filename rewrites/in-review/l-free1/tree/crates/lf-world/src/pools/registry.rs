//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit routine of the slot-descriptor structure:
//! the context cluster, the seven cursor-step instances, the validity
//! check, the release, the three scans and the two script-pool routines.
//! `Proven` means the routine is restated on [`SlotPool`](crate::pools::SlotPool)
//! and the differential test crate ran it against its verified rewrite on
//! the same generated inputs, comparing results and every effect, with a
//! deliberately wrong lift caught alongside. Counts below come from this
//! table.
//!
//! The other pool structures in the lane's list (the pool vectors, the
//! mutex-guarded pools, the row/cell table pools, the inline record
//! arrays) have no rows yet: later lanes add one section each.

/// Lift state of one verified routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its pool type and proven against its rewrite.
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
    /// Naming-lane name of the verified routine, e.g. `"pool_slot_occupied"`.
    pub func: &'static str,
    /// Lifted method, e.g. `"SlotPool::is_occupied"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the routine is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified routine of the slot-descriptor structure.
pub const ROWS: &[Row] = &[
    Row {
        func: "pool_context_create",
        method: "SlotPool::create_ctx",
        state: State::Proven,
        narrows: &[
            "the context global becomes the return value; the 28-byte block stays opaque (its initialiser is not verified)",
        ],
    },
    Row {
        func: "pool_slot_occupied",
        method: "SlotPool::is_occupied",
        state: State::Proven,
        narrows: &[
            "the slot-address non-null guard is always true over owned entries",
            "indexes past the flag store panic; the original reads past it",
        ],
    },
    Row {
        func: "pool_slot_data_word",
        method: "SlotPool::data_word",
        state: State::Proven,
        narrows: &[
            "dead slots panic; the original reads through null and faults",
            "reads past the entry store panic; the original reads on",
        ],
    },
    Row {
        func: "pool_slot_assign",
        method: "SlotPool::assign",
        state: State::Proven,
        narrows: &[
            "dead slots panic; the original stores through null and faults",
            "the 1/0 answer narrows to bool",
        ],
    },
    Row {
        func: "pool_indexed_store",
        method: "SlotPool::indexed_store",
        state: State::Proven,
        narrows: &[
            "the refresh answer translates from a row base address to a row index",
            "dead slots panic; the original faults through null",
            "cell indexes past the table panic; the original reads past it",
            "proof layouts avoid address wrap (checked per case)",
        ],
    },
    Row {
        func: "pool_slot_valid_check",
        method: "SlotPool::slot_at_offset",
        state: State::Proven,
        narrows: &[
            "the absolute slot address narrows to its offset from the entry base",
            "the residue-carrying 1/0 answer narrows to Option<usize> (the proof pins the low byte and the index)",
            "offsets that wrap the address below the base are out of domain",
            "stride 0 and the MIN/-1 division panic; the original faults",
            "quotients resolving outside the flag store panic; the original reads there",
        ],
    },
    Row {
        func: "pool_iterator_16f7d60",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &[
            "the slot-address-or-null answer narrows to Option<usize>; the proof reconstructs the address per case",
            "cursors above the slot count panic; the original reads past the flag store",
            "the null-slot guard never fires on proof layouts (checked per case)",
            "the live count is the flag store length",
        ],
    },
    Row {
        func: "pool_iterator_12bd0e8",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &["same routine as pool_iterator_16f7d60 over its own record; same narrowings"],
    },
    Row {
        func: "pool_iterator_166d9ec",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &["same routine as pool_iterator_16f7d60 over its own record; same narrowings"],
    },
    Row {
        func: "pool_iterator_18b6f10",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &["same routine as pool_iterator_16f7d60 over its own record; same narrowings"],
    },
    Row {
        func: "pool_iterator_1632c60",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &["same routine as pool_iterator_16f7d60 over its own record; same narrowings"],
    },
    Row {
        func: "pool_iterator_18b6f1c",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &["same routine as pool_iterator_16f7d60 over its own record; same narrowings"],
    },
    Row {
        func: "pool_iterator_12e22a4",
        method: "SlotPool::cursor_step",
        state: State::Proven,
        narrows: &["same routine as pool_iterator_16f7d60 over its own record; same narrowings"],
    },
    Row {
        func: "pool_subsystem_init",
        method: "-",
        state: State::Missing,
        narrows: &[
            "builds the subsystem descriptor from code addresses and allocator probes through four callees; needs a handler-trait design and the callee meanings",
        ],
    },
    Row {
        func: "pool_slot_release",
        method: "-",
        state: State::Missing,
        narrows: &["refcount release with survives/evict callees; same structure, not reached"],
    },
    Row {
        func: "pool_scan_and_report",
        method: "-",
        state: State::Missing,
        narrows: &["top-down scan with a stack-cookie mirror and worker callees; not reached"],
    },
    Row {
        func: "pool_collect_farthest_capped",
        method: "-",
        state: State::Missing,
        narrows: &["capped farthest-first collect over float distances; not reached"],
    },
    Row {
        func: "pool_group_flag_distant_idle",
        method: "-",
        state: State::Missing,
        narrows: &["group scan flagging far idle members; not reached"],
    },
    Row {
        func: "pool_find_matching_slot",
        method: "-",
        state: State::Missing,
        narrows: &[
            "the tracked rewrite does not compile from the repository (undefined pool constants; open review issue), so it cannot be differentially proven until fixed",
        ],
    },
    Row {
        func: "pool_append_converted_string",
        method: "-",
        state: State::Missing,
        narrows: &[
            "same structure, same uncompilable rewrite as pool_find_matching_slot; not reached",
        ],
    },
];

/// Number of rows in a state.
#[must_use]
pub const fn count(state: State) -> usize {
    let mut n = 0;
    let mut i = 0;
    while i < ROWS.len() {
        if ROWS[i].state as u8 == state as u8 {
            n += 1;
        }
        i += 1;
    }
    n
}
