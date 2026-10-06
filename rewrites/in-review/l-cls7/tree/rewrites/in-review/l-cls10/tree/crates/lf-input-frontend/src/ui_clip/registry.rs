//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the two classes. `Proven`
//! means the method is restated on [`BasicClip`](super::BasicClip) and
//! the differential test crate ran it against its verified rewrite on
//! the same generated inputs, comparing results, every written byte
//! and every collaborator call in order, with a deliberately wrong lift
//! caught alongside. Counts below come from this table.

/// Lift state of one verified method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its clip type and proven against its rewrite.
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
    /// Class, e.g. `"UIBasicClip"`.
    pub class: &'static str,
    /// Slot or method name in the 32-bit form, e.g. `"vf142"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the method is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified method of the family, in class order.
pub const ROWS: &[Row] = &[
    // The basic clip: accessors.
    Row {
        class: "UIBasicClip",
        method: "vf142",
        state: State::Proven,
        narrows: &["byte answer travels as u8 (the original's upper result bits are residue)"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf139",
        state: State::Proven,
        narrows: &["sink address travels as an opaque cookie"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf148",
        state: State::Proven,
        narrows: &[
            "float-stack return travels as f32; its signalling-NaN quieting is reproduced as bit operations",
        ],
    },
    // The basic clip: two-level word traffic through the submit part.
    Row {
        class: "UIBasicClip",
        method: "vf147",
        state: State::Proven,
        narrows: &[
            "out-pointer answer dropped (the write through it is compared)",
            "submit part travels as an opaque cookie; missing panics where the original faults",
        ],
    },
    Row {
        class: "UIBasicClip",
        method: "vf130",
        state: State::Proven,
        narrows: &[
            "source pointer narrows to its word",
            "submit part travels as an opaque cookie; missing panics where the original faults",
        ],
    },
    // The basic clip: probe, conditional forward, flag loop.
    Row {
        class: "UIBasicClip",
        method: "vf123",
        state: State::Proven,
        narrows: &["own unlifted slots travel as world calls"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf131",
        state: State::Proven,
        narrows: &[
            "predicate answer narrows to its low byte",
            "parts travel as opaque cookies; missing panics where the original faults",
        ],
    },
    Row {
        class: "UIBasicClip",
        method: "vf124",
        state: State::Proven,
        narrows: &[
            "flag word narrows to its low byte",
            "part count must stay inside the element array (panics past it)",
            "elements travel as opaque cookies",
        ],
    },
    // The basic clip: float arithmetic through collaborators.
    Row {
        class: "UIBasicClip",
        method: "vf129",
        state: State::Proven,
        narrows: &[
            "shared globals travel as pinned constants",
            "parts travel as opaque cookies; missing panics where the original faults",
        ],
    },
    Row {
        class: "UIBasicClip",
        method: "vf132",
        state: State::Proven,
        narrows: &[
            "shared globals and the engine table travel as a pinned constant and a kind tag",
            "frame-pointer arguments skipped (stack addresses); the scratch word is always zero",
            "frame check carried as a world call so the result word compares",
        ],
    },
    Row {
        class: "UIBasicClip",
        method: "vf137",
        state: State::Proven,
        narrows: &[
            "shared globals and the engine table travel as a pinned constant and a kind tag",
            "frame-pointer arguments skipped (stack addresses); the scratch word is always zero",
            "frame check carried as a world call so the result word compares",
        ],
    },
    // The basic clip: text traffic.
    Row {
        class: "UIBasicClip",
        method: "vf126",
        state: State::Proven,
        narrows: &[
            "mode word narrows to a flag; strings arrive as slices holding a NUL within bounds",
            "scratch addresses skipped; the full 256 scratch bytes are compared instead",
            "scratch zeroing modelled by initialization; frame check dropped (answer unused)",
            "child travels as an opaque cookie; missing panics where the original faults",
        ],
    },
    Row {
        class: "UIBasicClip",
        method: "vf138",
        state: State::Proven,
        narrows: &[
            "flag word narrows to a flag; strings arrive as slices holding a NUL within bounds",
            "scratch addresses skipped; the full 256 scratch bytes are compared instead",
            "frame check dropped (answer unused); zero return narrows to ()",
            "sink and parts travel as opaque cookies; missing panics where the original faults",
        ],
    },
    // The basic clip: display-mode switch.
    Row {
        class: "UIBasicClip",
        method: "vf133",
        state: State::Proven,
        narrows: &[
            "mode word narrows to its low byte",
            "matcher out-pointer skipped (stack address); its words travel as the match answer",
            "scratch and table addresses travel as opaque cookies",
            "parts travel as opaque cookies; missing panics where the original faults",
        ],
    },
    // The basic clip: pure forwards carry no behaviour and are not lifted.
    Row {
        class: "UIBasicClip",
        method: "vf0",
        state: State::Missing,
        narrows: &["one helper call with a fixed argument: no behaviour to restate"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf2",
        state: State::Missing,
        narrows: &["deleting destructor: Drop covers it"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf125",
        state: State::Missing,
        narrows: &["loads the inner object and jumps to the shared worker: one forwarded call"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf127",
        state: State::Missing,
        narrows: &["tail-jump to one slot of the inner object: one forwarded call"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf128",
        state: State::Missing,
        narrows: &["forwards two arguments to one member slot: one forwarded call"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf134",
        state: State::Missing,
        narrows: &["forwards one argument to one member slot: one forwarded call"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf135",
        state: State::Missing,
        narrows: &["forwards one argument to one member slot: one forwarded call"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf136",
        state: State::Missing,
        narrows: &["forwards two arguments to one member slot: one forwarded call"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf140",
        state: State::Missing,
        narrows: &["forwards one argument to one member slot: one forwarded call"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf143",
        state: State::Missing,
        narrows: &["member-address query: the pointee layout is unknown, so no field can own it"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf144",
        state: State::Missing,
        narrows: &["tail-jump to one slot of the part object: one forwarded call"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf145",
        state: State::Missing,
        narrows: &["forwards one argument to one member slot: one forwarded call"],
    },
    Row {
        class: "UIBasicClip",
        method: "vf146",
        state: State::Missing,
        narrows: &["forwards one argument to one member slot: one forwarded call"],
    },
    // The replay progress bar: not lifted in this lane.
    Row {
        class: "CReplayProgressBar",
        method: "CReplayProgressBar_2",
        state: State::Missing,
        narrows: &["teardown sequencing through token, slot-release and finalizer helpers: needs a slot-release world"],
    },
    Row {
        class: "CReplayProgressBar",
        method: "vf0",
        state: State::Missing,
        narrows: &["deleting destructor: Drop covers it"],
    },
    Row {
        class: "CReplayProgressBar",
        method: "vf1",
        state: State::Missing,
        narrows: &["update through twenty callees with out-pointer helpers and five globals: needs the update world and its state struct"],
    },
    Row {
        class: "CReplayProgressBar",
        method: "vf2",
        state: State::Missing,
        narrows: &["progress draw through display, duration, palette and draw-list helpers: needs the drawing world and its state struct"],
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
