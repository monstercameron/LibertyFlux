//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the string class. `Proven`
//! means the method is restated on [`FontString`](super::FontString) and
//! the differential test crate ran it against its verified rewrite on
//! the same generated inputs, comparing results, every written byte
//! and every world call in order, with a deliberately wrong lift
//! caught alongside. Counts below come from this table.

/// Lift state of one verified method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its string type and proven against its rewrite.
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
    /// Class, e.g. `"UIFontString"`.
    pub class: &'static str,
    /// Slot name in the 32-bit form, e.g. `"vf133"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the method is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified method of the family, in slot order.
pub const ROWS: &[Row] = &[
    // The deleting destructor carries no behaviour.
    Row {
        class: "UIFontString",
        method: "vf2",
        state: State::Missing,
        narrows: &["deleting destructor: Drop covers it"],
    },
    // The measure pass.
    Row {
        class: "UIFontString",
        method: "vf83",
        state: State::Proven,
        narrows: &[
            "UI mode bytes, scale triple, display extents and slot selector travel as arguments",
            "live text must hold a NUL within its 256 bytes",
            "scratch and converted-word addresses skipped; the eight words are compared instead",
            "cached pointer skipped; the eight words behind it are compared instead",
            "hook answers arrive through the float-stack return; its signalling-NaN quieting is reproduced in the proof",
        ],
    },
    // The refresh-and-render pass.
    Row {
        class: "UIFontString",
        method: "vf85",
        state: State::Proven,
        narrows: &[
            "scale global travels as an argument",
            "hook answers arrive through the float-stack return; its signalling-NaN quieting is reproduced in the proof",
        ],
    },
    // The row snapshot.
    Row {
        class: "UIFontString",
        method: "vf86",
        state: State::Proven,
        narrows: &[
            "slot selector travels as an argument; slots past the rows panic",
            "live text must hold a NUL within its 256 bytes",
        ],
    },
    // The row emitter.
    Row {
        class: "UIFontString",
        method: "vf87",
        state: State::Proven,
        narrows: &[
            "slot selector and UI mode bytes travel as arguments; slots past the rows panic",
            "row text must hold a NUL within its 256 bytes",
            "scratch and converted-word addresses skipped; the eight words are compared instead",
            "cached pointer skipped; the eight words behind it are compared instead",
        ],
    },
    // The reset.
    Row {
        class: "UIFontString",
        method: "vf115",
        state: State::Proven,
        narrows: &[
            "source pointer narrows to its word",
            "default-float global travels as an argument",
            "backend pointer skipped; the four snapshot words are compared instead",
        ],
    },
    // The handle setters.
    Row {
        class: "UIFontString",
        method: "vf117",
        state: State::Proven,
        narrows: &[
            "scratch pointer skipped; its snapshot is the two zero words on both sides",
            "backend pointer skipped; the four snapshot words are compared instead",
        ],
    },
    Row {
        class: "UIFontString",
        method: "vf118",
        state: State::Proven,
        narrows: &[
            "scratch slot address skipped; the size argument selects the branch and is compared",
            "scratch snapshot is the two zero words on both sides",
            "backend pointer skipped; the four snapshot words are compared instead",
        ],
    },
    Row {
        class: "UIFontString",
        method: "vf119",
        state: State::Proven,
        narrows: &[
            "flag word narrows to the hold bit",
            "backend pointer skipped; the four snapshot words are compared instead",
        ],
    },
    // The text setter.
    Row {
        class: "UIFontString",
        method: "vf120",
        state: State::Proven,
        narrows: &[
            "flag word narrows to the refresh bit",
            "text pointer narrows to the 256-byte buffer or nothing",
            "copier destination skipped (always the text buffer); the forced terminator is compared",
            "the copier's copy is the world's work: under the non-writing stub the buffer keeps its bytes",
        ],
    },
    // The float setters.
    Row {
        class: "UIFontString",
        method: "vf121",
        state: State::Proven,
        narrows: &["float bits travel as f32, verbatim"],
    },
    Row {
        class: "UIFontString",
        method: "vf123",
        state: State::Proven,
        narrows: &["float bits travel as f32, verbatim"],
    },
    Row {
        class: "UIFontString",
        method: "vf124",
        state: State::Proven,
        narrows: &["float bits travel as f32, verbatim"],
    },
    // The flag setters.
    Row {
        class: "UIFontString",
        method: "vf125",
        state: State::Proven,
        narrows: &["argument narrows to its low byte"],
    },
    Row {
        class: "UIFontString",
        method: "vf127",
        state: State::Proven,
        narrows: &["argument narrows to its low byte"],
    },
    Row {
        class: "UIFontString",
        method: "vf128",
        state: State::Proven,
        narrows: &["argument narrows to its low byte"],
    },
    Row {
        class: "UIFontString",
        method: "vf129",
        state: State::Proven,
        narrows: &["argument narrows to its low byte"],
    },
    // The style-word pair.
    Row {
        class: "UIFontString",
        method: "vf130",
        state: State::Proven,
        narrows: &["source pointer narrows to its word"],
    },
    Row {
        class: "UIFontString",
        method: "vf133",
        state: State::Proven,
        narrows: &["out-pointer answer dropped (the write through it is compared)"],
    },
];

/// Counts of rows by state: `(proven, lifted, missing)`.
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
