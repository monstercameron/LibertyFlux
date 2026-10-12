//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per inventoried method of the hosting task. `Proven` means the
//! verified method is restated on [`HostSessionTask`] and the differential
//! test crate ran it against its verified rewrite on the same generated
//! inputs, comparing results, observed effects, and collaborator calls with
//! a deliberately wrong behavior variant caught alongside. Counts below
//! come from this table.
//!
//! [`HostSessionTask`]: super::task::HostSessionTask

/// Lift state of one inventoried method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on the task type and proven against its rewrite.
    Proven,
    /// Restated but not yet proven against its rewrite.
    Lifted,
    /// Not lifted, for the stated reason.
    Missing,
}

/// One inventoried method's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Task class, always `"HostSession"` here.
    pub channel: &'static str,
    /// Slot name in the 32-bit form, e.g. `"vf3"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away. Empty means the proof compares
    /// everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Inventoried methods of the hosting task and their current lift status.
pub const ROWS: &[Row] = &[
    Row {
        channel: "HostSession",
        method: "setup_gamer_data",
        state: State::Proven,
        narrows: &[
            "the unused first stack word is dropped",
            "the gamer source arrives as a 259-word array",
            "the trailer record length is set by the world; the proof compares the words it writes",
            "the stock contract has fulldata disabled",
        ],
    },
    Row {
        channel: "HostSession",
        method: "vf2",
        state: State::Proven,
        narrows: &[
            "the stock contract has fulldata disabled",
            "the session link is non-null in the proof inputs; the original reads through it unconditionally",
            "the own-completion virtual slot is represented by a world call",
        ],
    },
    Row {
        channel: "HostSession",
        method: "vf3",
        state: State::Proven,
        narrows: &[
            "callee 6's first stack pointer is not compared by value; four words at the pointed-to buffer are snapshotted",
            "callee behavior beyond the declared arguments, answers, snapshots, and task writes is not lifted",
            "the session link is non-null in the proof inputs; null-link behavior is not exercised",
        ],
    },
    Row {
        channel: "HostSession",
        method: "vf7",
        state: State::Proven,
        narrows: &[
            "relocated tags are the image base plus the file tag word",
            "callee ids 3, 8, and 9 omit the first stack pointer value and snapshot one pointed-to word each",
            "callee ids 4, 5, and 6 omit ECX pointer identity and snapshot one pointed-to word each",
            "callee id 10's ECX value is not observed by the stock contract",
            "descriptor self-pointers and padding are not compared as independent lift state",
            "callee behavior beyond the declared arguments, answers, snapshots, and task writes is not lifted",
            "the session link is non-null in the proof inputs; the original reads through it unconditionally",
        ],
    },
    Row {
        channel: "HostSession",
        method: "get_descriptor",
        state: State::Missing,
        narrows: &["listed by the inventory but unverified; no rewrite or lift is included"],
    },
];

/// Counts of (proven, lifted, missing) rows.
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
