//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the hosting task. `Proven` means
//! the method is restated on [`HostSessionTask`] and the differential test
//! crate ran it against its verified rewrite on the same generated inputs,
//! comparing results, every effect, and every collaborator call in order,
//! with a deliberately wrong lift caught alongside. Counts below come from
//! this table.
//!
//! [`HostSessionTask`]: super::task::HostSessionTask

/// Lift state of one verified method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on the task type and proven against its rewrite.
    Proven,
    /// Restated but not yet proven against its rewrite.
    Lifted,
    /// Not lifted, for the stated reason.
    Missing,
}

/// One verified method's row.
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

/// Every verified method of the hosting task.
pub const ROWS: &[Row] = &[
    Row {
        channel: "HostSession",
        method: "setup",
        state: State::Lifted,
        narrows: &[
            "the unused first stack word is dropped",
            "the gamer source arrives as a 259-word array",
            "the trailer record length is set by the world; the proof compares the words it writes",
        ],
    },
    Row {
        channel: "HostSession",
        method: "vf2",
        state: State::Lifted,
        narrows: &["the session link must be present; the original faults on a null link"],
    },
    Row {
        channel: "HostSession",
        method: "vf3",
        state: State::Lifted,
        narrows: &[
            "ignored callee answers are unmodelled",
            "task-side effects of the promote, notify and progress callees are unobserved by the proof and not modelled",
            "the loop repeats while the progress check answers nonzero, as in the original",
        ],
    },
    Row {
        channel: "HostSession",
        method: "vf7",
        state: State::Lifted,
        narrows: &[
            "relocated tags are the image base plus the file tag word",
            "descriptor self-pointers and padding stay inside the world",
            "zero-word pointers narrow to no argument",
            "ignored callee answers are unmodelled",
            "task-side effects of the notify callee are unobserved by the proof and not modelled",
            "the session link must be present; the original faults on a null link",
        ],
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
