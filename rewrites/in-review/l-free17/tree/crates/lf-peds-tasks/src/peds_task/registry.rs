//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit routine of the covered structures. `Proven`
//! means the routine is restated on its owning type and the differential
//! test crate ran it against its verified rewrite on the same generated
//! inputs, comparing results and every effect, with a deliberately wrong
//! lift caught alongside. Counts below come from this table.

/// Lift state of one verified routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on the owning type and proven against its rewrite.
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
    /// Owning structure short name, e.g. `"WeightedPicker"`.
    pub class: &'static str,
    /// Routine name in the 32-bit form, e.g. `"weighted_pick"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the routine is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified routine of the covered structures, grouped by structure.
pub const ROWS: &[Row] = &[
    // WeightedPicker: the weighted pick. Its structure's only routine in
    // the group; the shape is complete.
    Row {
        class: "WeightedPicker",
        method: "weighted_pick",
        state: State::Proven,
        narrows: &[
            "the entry records travel as the picked index (never read, only addressed for the fill call): the proof plants the 32-byte records and asserts the stub addresses",
            "the security-cookie check runs on the rewrite side as a counting stub and has no lifted counterpart: the proof asserts it ran exactly once",
        ],
    },
    // FacingQuery: the reaction code. Its structure's only routine in the
    // group; the shape is complete.
    Row {
        class: "FacingQuery",
        method: "react_code",
        state: State::Proven,
        narrows: &[
            "the task's matrix-or-inline position choice is parsed by the caller: the proof plants both layouts",
            "the mode word narrows to the 3-or-4 bit, the flag byte to a bool, the counter pointers to the counter value: the proof plants the words and compares the parsed fields",
            "the threshold and the four classifier bounds are caller-supplied tuning: the proof varies them per case",
            "the ped-state probe travels as the PedState trait: the proof scripts answers on both sides and compares the planted object address and the constant argument",
        ],
    },
    // TaskStateBlock: the build/consume pair. Both routines of this
    // structure in the group are lifted; the shape is complete.
    Row {
        class: "TaskStateBlock",
        method: "state_init",
        state: State::Proven,
        narrows: &[
            "the two scratch words the rewrite pins to zero are not modelled: the proof asserts they read zero",
            "the enumeration block's trailing zero and radius, the consumer callback address and the trailing 0/4/5 travel as constants of the Enumerate trait: the proof reads the block words through the stub pointer and compares them with the lifted midpoints",
            "the enumerator receives the state because it may re-enter the flag: the proof scripts a re-entering stub on some cases and compares the answer",
            "the copy source is another subsystem's word, carried as a field: the proof plants it as the global and the field",
        ],
    },
    Row {
        class: "TaskStateBlock",
        method: "state_consumer",
        state: State::Proven,
        narrows: &[
            "the constant 1 answer narrows to a u8: the proof compares the low byte and scripts check answers with nonzero upper bytes",
            "the task object travels as an opaque handle plus its vector: the proof plants the vtable and the record and asserts the forwarded identity",
            "the vtable slot's fourth and fifth answered words are never read (the fifth only addressed for the worker call): the proof scripts them arbitrarily",
            "the worker's scratch address is a stack address with no lifted counterpart: the proof records it and compares the object, the v2 word, the quarter and the six zeros",
            "the check call's two addresses are fixed state offsets: the proof asserts the stub received the planted cells",
            "the security-cookie check runs on the rewrite side as a counting stub and has no lifted counterpart: the proof asserts it ran exactly once",
            "callees are scripted to leave the state block untouched: the proof compares every state word after each case",
        ],
    },
];

/// Counts of (proven, lifted-but-unproven, missing) rows.
#[must_use]
pub fn counts() -> (usize, usize, usize) {
    let mut proven = 0;
    let mut lifted = 0;
    let mut missing = 0;
    for row in ROWS {
        match row.state {
            State::Proven => proven += 1,
            State::Lifted => lifted += 1,
            State::Missing => missing += 1,
        }
    }
    (proven, lifted, missing)
}
