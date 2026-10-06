//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the two lifted classes.
//! `Proven` means the method is restated on its type and the differential
//! test crate ran it against its verified rewrite on the same generated
//! inputs, comparing results, every written byte, and every collaborator
//! call in order, with a deliberately wrong lift caught alongside.
//! Counts below come from this table.

/// Lift state of one verified method.
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

/// One verified method's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Class, e.g. `"StreamDevice"`.
    pub class: &'static str,
    /// Slot or method name in the 32-bit form, e.g. `"vf27"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the method is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified method of the lifted classes, in class order.
pub const ROWS: &[Row] = &[
    // Streaming device: seven proven; the pure forward and the deleting
    // destructor carry no behaviour.
    Row {
        class: "StreamDevice",
        method: "vf27",
        state: State::Proven,
        narrows: &["1/-1 answer narrows to bool"],
    },
    Row {
        class: "StreamDevice",
        method: "vf19",
        state: State::Proven,
        narrows: &["64-bit answer narrows to its low word; the cleared high half is pinned"],
    },
    Row {
        class: "StreamDevice",
        method: "vf20",
        state: State::Proven,
        narrows: &[
            "null resolution panics: the original faults there",
            "kinds stay inside the tables",
        ],
    },
    Row {
        class: "StreamDevice",
        method: "vf33",
        state: State::Missing,
        narrows: &["one forwarded call: no behaviour in it"],
    },
    Row {
        class: "StreamDevice",
        method: "vf29",
        state: State::Proven,
        narrows: &["-1/0 answer plus the written word narrow to Option"],
    },
    Row {
        class: "StreamDevice",
        method: "vf1",
        state: State::Proven,
        narrows: &[
            "the unread second argument is dropped",
            "null resolution panics: the original faults there",
            "kinds stay inside the tables",
        ],
    },
    Row {
        class: "StreamDevice",
        method: "vf2",
        state: State::Proven,
        narrows: &[
            "the always-zero second output word is not modelled",
            "the entry offset arrives with the entry: the global table base is not modelled",
            "kinds stay inside the tables",
        ],
    },
    Row {
        class: "StreamDevice",
        method: "vf6",
        state: State::Proven,
        narrows: &[
            "elements and kinds stay inside the owned tables",
            "the channel row is read once: a close call rewriting it is not modelled",
        ],
    },
    Row {
        class: "StreamDevice",
        method: "deleting",
        state: State::Missing,
        narrows: &["deleting destructor through the heap: Drop covers it"],
    },
    // Tokenizer reads.
    Row {
        class: "Tokenizer",
        method: "read_until_delimiter",
        state: State::Proven,
        narrows: &[
            "the caller buffer address is not modelled: written bytes compare exactly",
            "the refill cell address is not compared",
            "stream positions stay inside the buffered bytes",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf5",
        state: State::Proven,
        narrows: &[
            "the fetch buffer address is not compared; its length and bytes are",
            "scripted tokens carry no interior NUL",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf6",
        state: State::Proven,
        narrows: &[
            "the fetch buffer address is not compared; its length and bytes are",
            "scripted tokens carry no interior NUL",
            "required=false is unproven: the original loads through an address the checker cannot serve",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf11",
        state: State::Proven,
        narrows: &["the last-bits answer narrows to the values"],
    },
    Row {
        class: "Tokenizer",
        method: "vf10",
        state: State::Proven,
        narrows: &["the last-bits answer narrows to the values"],
    },
    Row {
        class: "Tokenizer",
        method: "vf9",
        state: State::Proven,
        narrows: &["the last-bits answer narrows to the values"],
    },
    Row {
        class: "Tokenizer",
        method: "vf8",
        state: State::Proven,
        narrows: &["the out-pointer answer narrows to the values"],
    },
    Row {
        class: "Tokenizer",
        method: "vf7",
        state: State::Proven,
        narrows: &["the out-pointer answer narrows to the values"],
    },
    Row {
        class: "Tokenizer",
        method: "vf12",
        state: State::Proven,
        narrows: &["the fetch buffer address is not compared"],
    },
    Row {
        class: "Tokenizer",
        method: "vf13",
        state: State::Proven,
        narrows: &["the fetch buffer address is not compared"],
    },
    Row {
        class: "Tokenizer",
        method: "vf14",
        state: State::Proven,
        narrows: &["the fetch buffer address is not compared"],
    },
    Row {
        class: "Tokenizer",
        method: "vf19",
        state: State::Proven,
        narrows: &["the fetch buffer address is not compared"],
    },
    Row {
        class: "Tokenizer",
        method: "vf18",
        state: State::Proven,
        narrows: &["the fetch buffer address is not compared"],
    },
    Row {
        class: "Tokenizer",
        method: "vf17",
        state: State::Proven,
        narrows: &["the fetch buffer address is not compared"],
    },
    Row {
        class: "Tokenizer",
        method: "vf16",
        state: State::Proven,
        narrows: &["the fetch buffer address is not compared"],
    },
    Row {
        class: "Tokenizer",
        method: "vf15",
        state: State::Proven,
        narrows: &["the fetch buffer address is not compared"],
    },
    Row {
        class: "Tokenizer",
        method: "vf20",
        state: State::Proven,
        narrows: &[
            "the fetch buffer address is not compared",
            "scripted tokens carry no interior NUL",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf21",
        state: State::Proven,
        narrows: &[
            "the fetch buffer address is not compared",
            "scripted tokens carry no interior NUL",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf26",
        state: State::Proven,
        narrows: &[
            "the fetch buffer address is not compared",
            "scripted tokens carry no interior NUL",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf25",
        state: State::Proven,
        narrows: &[
            "the fetch buffer address is not compared",
            "scripted tokens carry no interior NUL",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf24",
        state: State::Proven,
        narrows: &[
            "the fetch buffer address is not compared",
            "scripted tokens carry no interior NUL",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf23",
        state: State::Proven,
        narrows: &[
            "the fetch buffer address is not compared",
            "scripted tokens carry no interior NUL",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf22",
        state: State::Proven,
        narrows: &[
            "the fetch buffer address is not compared",
            "scripted tokens carry no interior NUL",
        ],
    },
    // Tokenizer writes.
    Row {
        class: "Tokenizer",
        method: "vf27",
        state: State::Proven,
        narrows: &[
            "the untouched return register is not modelled",
            "the slow-path cell's stale high bytes are not modelled: only the byte is compared",
            "stream positions stay inside the buffered bytes",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf28",
        state: State::Proven,
        narrows: &[
            "the untouched return register is not modelled",
            "the slow-path cell's stale high bytes are not modelled: only the byte is compared",
            "stream positions stay inside the buffered bytes",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf29",
        state: State::Proven,
        narrows: &["the untouched return register is not modelled"],
    },
    Row {
        class: "Tokenizer",
        method: "vf30",
        state: State::Proven,
        narrows: &["the untouched return register is not modelled"],
    },
    Row {
        class: "Tokenizer",
        method: "vf37",
        state: State::Proven,
        narrows: &[
            "only the low byte of the answer is meaningful: it narrows to bool",
            "empty text is unproven: the original selects a default through an address the checker cannot serve",
            "text is NUL-terminated inside the slice",
            "the slow-path cell's stale high bytes are not modelled: only the byte is compared",
            "stream positions stay inside the buffered bytes",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf39",
        state: State::Proven,
        narrows: &[
            "only the low byte of the answer is meaningful: it narrows to bool",
            "the format length equals the formatted bytes",
            "the unrelocated format-string argument is not compared",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "vf40",
        state: State::Proven,
        narrows: &[
            "only the low byte of the answer is meaningful: it narrows to bool",
            "the format length equals the formatted bytes",
            "the unrelocated format-string argument is not compared",
        ],
    },
    Row {
        class: "Tokenizer",
        method: "ctor",
        state: State::Missing,
        narrows: &["in-place constructor over a caller stream: new() covers it"],
    },
];

/// Number of rows in a state.
#[must_use]
pub fn count(state: State) -> usize {
    ROWS.iter().filter(|row| row.state == state).count()
}
