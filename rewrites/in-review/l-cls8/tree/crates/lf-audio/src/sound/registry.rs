//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the two lifted effect classes.
//! `Proven` means the method is restated on its effect type and the
//! differential test crate ran it against its verified rewrite on the
//! same generated inputs, comparing results, every effect, and every
//! collaborator call in order, with a deliberately wrong lift caught
//! alongside. Counts below come from this table.

/// Lift state of one verified method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its effect type and proven against its rewrite.
    Proven,
    /// Restated but not proven (no row should stay here: everything
    /// lifted in this module is proven).
    Lifted,
    /// Not lifted, for the stated reason.
    Missing,
}

/// One verified method's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Effect class, e.g. `"Effect"`.
    pub channel: &'static str,
    /// Slot name in the 32-bit form, e.g. `"vf5"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the method is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified method of the lifted classes, in class order.
pub const ROWS: &[Row] = &[
    // Base effect: five lifted, one missing.
    Row {
        channel: "Effect",
        method: "ctor",
        state: State::Proven,
        narrows: &[
            "the index words and gain slots the constructor leaves untouched are zeroed",
        ],
    },
    Row {
        channel: "Effect",
        method: "ctor2",
        state: State::Proven,
        narrows: &["the meaningless return word is unmodelled"],
    },
    Row {
        channel: "Effect",
        method: "dtor",
        state: State::Missing,
        narrows: &["allocator plumbing: Drop covers it"],
    },
    Row {
        channel: "Effect",
        method: "vf5",
        state: State::Proven,
        narrows: &[
            "copies must stay inside the table plus its trailing word, else panic",
        ],
    },
    Row {
        channel: "Effect",
        method: "vf1",
        state: State::Proven,
        narrows: &[
            "the answer's address residue narrows to a bool; the tag word arrives decoded",
        ],
    },
    Row {
        channel: "Effect",
        method: "vf2",
        state: State::Proven,
        narrows: &[
            "the meaningless return word is unmodelled; the derived pointer narrows to a slot index",
        ],
    },
    // Compressor effect: four lifted, one missing.
    Row {
        channel: "Compressor",
        method: "dtor",
        state: State::Missing,
        narrows: &["allocator plumbing: Drop covers it"],
    },
    Row {
        channel: "Compressor",
        method: "vf5",
        state: State::Proven,
        narrows: &["the index must select a row, else panic"],
    },
    Row {
        channel: "Compressor",
        method: "vf4",
        state: State::Proven,
        narrows: &[
            "the row address narrows to a reference; the index must select a row, else panic",
        ],
    },
    Row {
        channel: "Compressor",
        method: "vf1",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        channel: "Compressor",
        method: "vf2",
        state: State::Proven,
        narrows: &["the row address narrows to a word offset"],
    },
];

/// Counts of (proven, lifted-but-unproven, missing) rows.
#[must_use]
pub const fn counts() -> (usize, usize, usize) {
    let mut proven = 0;
    let mut lifted = 0;
    let mut missing = 0;
    let mut i = 0;
    while i < ROWS.len() {
        match ROWS[i].state {
            State::Proven => proven += 1,
            State::Lifted => lifted += 1,
            State::Missing => missing += 1,
        }
        i += 1;
    }
    (proven, lifted, missing)
}
