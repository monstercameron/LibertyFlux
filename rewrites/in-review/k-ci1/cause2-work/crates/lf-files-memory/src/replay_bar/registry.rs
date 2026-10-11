//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit routine of the group, in structure order:
//! the bar control's fifteen methods, the clock blend, the notifier
//! control, and the layout cell. `Proven` means the routine is restated
//! on its type and the differential test crate ran it against its
//! verified rewrite on the same generated inputs, comparing results,
//! every written byte, and every collaborator call in order, with a
//! deliberately wrong lift caught alongside. Counts below come from this
//! table.

/// Lift state of one verified routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its type and proven against its rewrite.
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
    /// Structure, e.g. `"ReplayBar"`.
    pub structure: &'static str,
    /// Routine name in the 32-bit form, e.g. `"scaled_index"`.
    pub routine: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the routine is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified routine of the group, in structure order.
pub const ROWS: &[Row] = &[
    // The bar control: seven call-free methods.
    Row {
        structure: "ReplayBar",
        routine: "scaled_index",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        structure: "ReplayBar",
        routine: "clamped_index",
        state: State::Proven,
        narrows: &["the flag word narrows to its decided low byte"],
    },
    Row {
        structure: "ReplayBar",
        routine: "find_slot",
        state: State::Proven,
        narrows: &[
            "the -1 miss answer narrows to None",
            "counts past the owned slots are out of domain (the 32-bit walk wraps there)",
        ],
    },
    Row {
        structure: "ReplayBar",
        routine: "millis_rounded",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        structure: "ReplayBar",
        routine: "hit_test",
        state: State::Proven,
        narrows: &["the 1/0 answer narrows to bool"],
    },
    Row {
        structure: "ReplayBar",
        routine: "region_hit_test",
        state: State::Proven,
        narrows: &["the 1/0 answer narrows to bool"],
    },
    Row {
        structure: "ReplayBar",
        routine: "store_cursor",
        state: State::Proven,
        narrows: &[],
    },
    // The bar control: six single-callee methods.
    Row {
        structure: "ReplayBar",
        routine: "scaled_position",
        state: State::Proven,
        narrows: &[
            "the mode word narrows to its decided low byte",
            "the f64 answer narrows to the f32 it widens",
        ],
    },
    Row {
        structure: "ReplayBar",
        routine: "scan_forward",
        state: State::Proven,
        narrows: &["the fixed bar pointer and trailing zero leave the scorer's arguments"],
    },
    Row {
        structure: "ReplayBar",
        routine: "scan_backward",
        state: State::Proven,
        narrows: &["the fixed bar pointer and trailing zero leave the scorer's arguments"],
    },
    Row {
        structure: "ReplayBar",
        routine: "measure_slots",
        state: State::Proven,
        narrows: &["the fixed bar pointer and trailing zero leave the scorer's arguments"],
    },
    Row {
        structure: "ReplayBar",
        routine: "slot_ratio",
        state: State::Proven,
        narrows: &["the echoed output pointer is pinned, not modelled"],
    },
    Row {
        structure: "ReplayBar",
        routine: "clamp_bound",
        state: State::Proven,
        narrows: &[],
    },
    // The bar control: two multi-callee methods.
    Row {
        structure: "ReplayBar",
        routine: "select_entry",
        state: State::Proven,
        narrows: &[
            "the empty-table address answer narrows to None (the proof rebuilds the address)",
            "the count word is the slot count",
            "selecting past the slots panics: the original reads past the table",
        ],
    },
    Row {
        structure: "ReplayBar",
        routine: "time_factor",
        state: State::Proven,
        narrows: &[
            "the mode words narrow to their decided low bytes",
            "the f64 answer narrows to the f32 it widens",
        ],
    },
    // The clock blend: a free function, no `this`.
    Row {
        structure: "ClockBlend",
        routine: "blend_factors",
        state: State::Proven,
        narrows: &[
            "the mode words narrow to their decided low bytes",
            "the echoed scale pointer is pinned, not modelled",
        ],
    },
    // The notifier control: its own `this` shape.
    Row {
        structure: "NotifyCtl",
        routine: "maybe_notify",
        state: State::Proven,
        narrows: &[
            "the discarded refresh answer is pinned, not modelled",
            "a null second fetch panics: the original faults there",
        ],
    },
    // The layout cell: a different structure.
    Row {
        structure: "BarLayout",
        routine: "layout",
        state: State::Missing,
        narrows: &[
            "a different object (offsets +0x00..+0x4C with flag bytes): not the bar",
            "seventeen callee slots and a dozen globals: needs its own trait design",
        ],
    },
];

/// Number of proven rows.
pub const PROVEN: usize = 17;
/// Number of rows.
pub const TOTAL: usize = 18;
