//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per named 32-bit method of the cutscene object class.
//! `Proven` means the method is restated on [`CutsceneObject`](super::CutsceneObject)
//! and the differential test crate ran it against its verified rewrite on
//! the same generated inputs, comparing results, every effect, and every
//! collaborator call in order, with a deliberately wrong lift caught
//! alongside. Counts below come from this table.

/// Lift state of one 32-bit method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on the cutscene object and proven against its rewrite.
    Proven,
    /// Restated but not proven (no row should stay here: everything
    /// lifted in this module is proven).
    Lifted,
    /// Not lifted, for the stated reason.
    Missing,
}

/// One 32-bit method's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Method name in the 32-bit form, e.g. `"vf27"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the method is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every named method of the class, in inventory order.
pub const ROWS: &[Row] = &[
    Row {
        method: "vf24",
        state: State::Missing,
        narrows: &["returns one address (the corner triple): no behaviour in it"],
    },
    Row {
        method: "dtor",
        state: State::Proven,
        narrows: &[
            "the destruction-phase table stamp is 32-bit plumbing: the proof checks it on the rewrite side only",
            "the registry table arrives as one word per index: the table itself is not modelled",
            "the fixed context address is pinned on the rewrite side, not modelled",
        ],
    },
    Row {
        method: "deleting",
        state: State::Missing,
        narrows: &["destructor plus conditional free through the allocator: Drop covers it"],
    },
    Row {
        method: "create_draw_commands",
        state: State::Proven,
        narrows: &[
            "the fourth stack word is never read: it is not a parameter",
            "the always-zero answer is not modelled",
            "the mode-2 flag check is the lifted predicate, not a world call",
            "the mode-1 mark lands on a world-owned target: the proof checks the rewrite's byte effect and the lift's mark call",
        ],
    },
    Row {
        method: "vf27",
        state: State::Proven,
        narrows: &[
            "out-pointer answer narrows to the written box",
            "the four shared-state words arrive as a parameter: no subsystem state yet",
            "the matrix callee's scratch-buffer address is not compared",
        ],
    },
    Row {
        method: "vf21",
        state: State::Proven,
        narrows: &["out-pointer answer narrows to the written triple"],
    },
    Row {
        method: "vf20",
        state: State::Proven,
        narrows: &[
            "the pose slot's scratch word is never read back: it is not modelled",
        ],
    },
    Row {
        method: "vf23",
        state: State::Proven,
        narrows: &[
            "the pose slot's scratch word is never read back: it is not modelled",
        ],
    },
    Row {
        method: "get_bound_radius",
        state: State::Proven,
        narrows: &["float-stack return travels as f32; its signalling-NaN quieting is reproduced as bit operations"],
    },
    Row {
        method: "vf26",
        state: State::Proven,
        narrows: &[
            "out-pointer answer narrows to the written rectangle",
            "the corner callee's out and vector addresses are not compared; the pushed corner values are",
            "the corner callee's third word is never read back: it is not modelled",
            "the unused third row of the fourth inline push is computed like the other rounds: unobservable",
        ],
    },
    Row {
        method: "get_max_bounds",
        state: State::Missing,
        narrows: &["returns one address (the corner triple): no behaviour in it"],
    },
    Row {
        method: "vf11",
        state: State::Proven,
        narrows: &["1/0 answer narrows to bool"],
    },
    Row {
        method: "vf9",
        state: State::Proven,
        narrows: &["1/0 answer narrows to bool"],
    },
    Row {
        method: "vf10",
        state: State::Proven,
        narrows: &["1/0 answer narrows to bool"],
    },
    Row {
        method: "is_field_2a0_nonzero",
        state: State::Proven,
        narrows: &["1/0 answer narrows to bool"],
    },
    Row {
        method: "vf18",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        method: "vf51",
        state: State::Missing,
        narrows: &[
            "the per-frame update: thirteen callees, two own-table guard slots, and about thirty shared words including bone tables and accumulators: needs those models, not built yet",
        ],
    },
    Row {
        method: "update_pose",
        state: State::Missing,
        narrows: &["unverified: there is no proven rewrite to lift from"],
    },
    Row {
        method: "refresh",
        state: State::Missing,
        narrows: &["unverified: there is no proven rewrite to lift from"],
    },
    Row {
        method: "vf34",
        state: State::Proven,
        narrows: &[
            "the hook's answer is dropped, like the original",
            "the both-flags-clear path answers zero, matching the rewrite (the original leaves its entry register: narrowed in the checker's contract too)",
        ],
    },
    Row {
        method: "vf54",
        state: State::Proven,
        narrows: &[],
    },
];

/// Number of rows in each state: (proven, lifted, missing).
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
