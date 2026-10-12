//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified or unverified 32-bit method of the base sound
//! class. `Proven` means the method is restated on [`AudSound`] (or as an
//! associated function of it) and the `lf-soundbase-diff` test crate ran it
//! against its verified rewrite on the same generated inputs, comparing the
//! result, every call with its arguments in order, and the bytes of the
//! test memory afterwards, with a deliberately wrong lift caught alongside.
//! `Missing` rows give the reason in `narrows`.

/// Lift state of one method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated and proven against its verified rewrite.
    Proven,
    /// Not lifted, for the stated reason.
    Missing,
}

/// One method's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Class the method belongs to.
    pub class: &'static str,
    /// Name in the 32-bit form and the lifted name, separated by a slash.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the method is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified or unverified method of the base sound class.
pub const ROWS: &[Row] = &[
    Row {
        class: "AudSound",
        method: "ctor / AudSound::new",
        state: State::Missing,
        narrows: &["no verified rewrite yet; the first constructor is unverified"],
    },
    Row {
        class: "AudSound",
        method: "audSound_2 / construct_second",
        state: State::Proven,
        narrows: &[
            "the init-table entry (callee 1) runs through the world's table read; the proof answers it with a stub that records its sub-id",
            "the cursor entry and the registry entry (callees 2 and 3) are scripted stubs",
            "the flag byte, sequence word and stored slots are compared; the fields outside AudSound (identity, option byte, voice pointer) are not written by this method and are not modelled",
        ],
    },
    Row {
        class: "AudSound",
        method: "~audSound__deleting / deleting_destructor (slot 5)",
        state: State::Proven,
        narrows: &[
            "the second constructor (callee 1) is a stub in this proof: its effect on the fields is proven by the construct_second row, and a port sequences the two",
            "the voice id is read before the second constructor runs; the second constructor never writes it",
        ],
    },
    Row {
        class: "AudSound",
        method: "vf8 / advance (slot 8)",
        state: State::Missing,
        narrows: &["verified, not lifted yet: writes voice records, rotates history and reads a divisor table; the next lane takes it"],
    },
    Row {
        class: "AudSound",
        method: "get_playback_start_ms",
        state: State::Missing,
        narrows: &["unverified: no verified rewrite yet"],
    },
    Row {
        class: "AudSound",
        method: "get_playback_end_ms",
        state: State::Missing,
        narrows: &["unverified: no verified rewrite yet"],
    },
    Row {
        class: "AudSound",
        method: "find_by_hash (slot 0)",
        state: State::Proven,
        narrows: &[
            "cursor objects, the link target and the answers of slot 0 are placed in a fixed window of the test memory; the slot-0 stub answers from a scripted queue",
            "voice ids are drawn from the first 32 rows of the voice table, which is what the test memory holds",
        ],
    },
    Row {
        class: "AudSound",
        method: "find_by_name / find_by_name (slot 1)",
        state: State::Proven,
        narrows: &[
            "the name hash (callee 1) is a stub; the proof checks its arguments (the name and a zero seed), not the hash it returns",
            "the slot-0 dispatch goes to the stub of the object's table",
        ],
    },
    Row {
        class: "AudSound",
        method: "vf7 / attach (slot 7)",
        state: State::Missing,
        narrows: &["verified, not lifted yet: it passes a scratch frame to three callees that write through it, and its callees need a frame model"],
    },
    Row {
        class: "AudSound",
        method: "vf2 / store_word_at_key (slot 2)",
        state: State::Proven,
        narrows: &[
            "the stored word is written into the test memory at the answer the slot-0 stub gives; a zero answer writes nothing",
        ],
    },
    Row {
        class: "AudSound",
        method: "vf3 / hash_and_forward (slot 3)",
        state: State::Proven,
        narrows: &["the hash (callee 1) is a stub with its arguments checked; the forwarded call is slot 2 of the object"],
    },
    Row {
        class: "AudSound",
        method: "vf4 / forward_link (slot 4)",
        state: State::Proven,
        narrows: &[
            "the fallback callee and the link target's slot 4 are stubs; the link target is always an object in the test window",
            "the link target is a raw word: a zero target faults in the original and is never generated",
        ],
    },
    Row {
        class: "AudSound",
        method: "vf6 / answers_false (slot 6)",
        state: State::Missing,
        narrows: &["the whole routine answers a constant zero: no behaviour to lift"],
    },
];

/// Counts of rows in a state: `(proven, missing)`.
#[must_use]
pub fn counts() -> (usize, usize) {
    let proven = ROWS.iter().filter(|r| r.state == State::Proven).count();
    (proven, ROWS.len() - proven)
}
