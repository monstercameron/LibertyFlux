//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the four lifted voice classes.
//! `Proven` means the method is restated on its voice type and the
//! differential test crate ran it against its verified rewrite on the
//! same generated inputs, comparing results, every effect, and every
//! collaborator call in order, with a deliberately wrong lift caught
//! alongside. The fifth class (`audVoicePhysical`) has no verified
//! methods yet and no rows here. Counts below come from this table.

/// Lift state of one verified method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its voice type and proven against its rewrite.
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
    /// Voice class, e.g. `"Soft"`.
    pub channel: &'static str,
    /// Slot name in the 32-bit form, e.g. `"vf4"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the method is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified method of the lifted classes, in class order.
pub const ROWS: &[Row] = &[
    // Software mixer voice: nine lifted, two missing.
    Row {
        channel: "Soft",
        method: "vf6",
        state: State::Proven,
        narrows: &["1/0 answer narrows to bool"],
    },
    Row {
        channel: "Soft",
        method: "vf7",
        state: State::Proven,
        narrows: &["1/0 answer narrows to bool", "cursor must select a lane"],
    },
    Row {
        channel: "Soft",
        method: "vf5",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        channel: "Soft",
        method: "vf4",
        state: State::Proven,
        narrows: &["the restart entry's ignored answer is unmodelled"],
    },
    Row {
        channel: "Soft",
        method: "vf3",
        state: State::Proven,
        narrows: &[
            "the level word travels as owned data, read once: the rewrite reads it twice around the level call",
        ],
    },
    Row {
        channel: "Soft",
        method: "vf2",
        state: State::Proven,
        narrows: &["the shutdown call's constant zero word is dropped"],
    },
    Row {
        channel: "Soft",
        method: "vf11",
        state: State::Proven,
        narrows: &[
            "the level word travels as owned data",
            "the fallback slot's ignored answer is unmodelled",
        ],
    },
    Row {
        channel: "Soft",
        method: "vf10",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        channel: "Soft",
        method: "vf8",
        state: State::Proven,
        narrows: &[
            "source words arrive as a slice",
            "cursor must select a lane; divisor must be nonzero",
            "the cursor is read once: the rewrite re-reads it after the float section, with no call between",
        ],
    },
    Row {
        channel: "Soft",
        method: "vf0",
        state: State::Missing,
        narrows: &[
            "deleting destructor through the voice pool: Drop covers it, the pool needs its own state struct",
        ],
    },
    Row {
        channel: "Soft",
        method: "vf1",
        state: State::Missing,
        narrows: &[
            "attach through mixer, allocator and effect-chain globals: needs those subsystem traits",
        ],
    },
    // PC ADPCM voice: eight lifted, two missing.
    Row {
        channel: "PcAdpcm",
        method: "vf6",
        state: State::Proven,
        narrows: &["1/0 answer narrows to bool"],
    },
    Row {
        channel: "PcAdpcm",
        method: "vf8",
        state: State::Proven,
        narrows: &[
            "source words arrive as a slice",
            "cursor must select a lane; divisor must be nonzero",
            "resolved table entries must stay in the owned tables",
            "the cursor is read once: collaborators borrow the world, never the voice",
        ],
    },
    Row {
        channel: "PcAdpcm",
        method: "vf10",
        state: State::Proven,
        narrows: &["the mixer word travels as a snapshot refreshed by the caller"],
    },
    Row {
        channel: "PcAdpcm",
        method: "vf3",
        state: State::Proven,
        narrows: &[
            "the level word travels as owned data, read once: the rewrite reads it twice around the level call",
        ],
    },
    Row {
        channel: "PcAdpcm",
        method: "vf2",
        state: State::Proven,
        narrows: &["the shutdown call's constant zero word is dropped"],
    },
    Row {
        channel: "PcAdpcm",
        method: "vf5",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        channel: "PcAdpcm",
        method: "vf4",
        state: State::Proven,
        narrows: &["the restart entry's ignored answer is unmodelled"],
    },
    Row {
        channel: "PcAdpcm",
        method: "vf11",
        state: State::Proven,
        narrows: &[
            "the level word travels as owned data",
            "the fallback slot's ignored answer is unmodelled",
        ],
    },
    Row {
        channel: "PcAdpcm",
        method: "vf0",
        state: State::Missing,
        narrows: &[
            "deleting destructor through the voice pool: Drop covers it, the pool needs its own state struct",
        ],
    },
    Row {
        channel: "PcAdpcm",
        method: "vf1",
        state: State::Missing,
        narrows: &[
            "attach through mixer, allocator and effect-chain globals: needs those subsystem traits",
        ],
    },
    // DirectSound voice: seven lifted, four missing.
    Row {
        channel: "DSound",
        method: "vf6",
        state: State::Proven,
        narrows: &["1/0 answer narrows to bool"],
    },
    Row {
        channel: "DSound",
        method: "vf7",
        state: State::Proven,
        narrows: &["1/0 answer narrows to bool", "cursor must select a window"],
    },
    Row {
        channel: "DSound",
        method: "vf4",
        state: State::Proven,
        narrows: &[
            "the resume call's constant zero words are dropped; the loop flag narrows to bool",
            "the restart position travels by value: the rewrite hands its address to the device",
            "the device table is read once: collaborators borrow the world, never the voice",
        ],
    },
    Row {
        channel: "DSound",
        method: "vf10",
        state: State::Proven,
        narrows: &["the play cursor travels by value from the device call"],
    },
    Row {
        channel: "DSound",
        method: "vf8",
        state: State::Proven,
        narrows: &[
            "source words arrive as a slice",
            "cursor must select a window; divisor must be nonzero",
        ],
    },
    Row {
        channel: "DSound",
        method: "vf3",
        state: State::Proven,
        narrows: &["the level word travels as owned data"],
    },
    Row {
        channel: "DSound",
        method: "vf2",
        state: State::Proven,
        narrows: &[],
    },
    Row {
        channel: "DSound",
        method: "vf0",
        state: State::Missing,
        narrows: &[
            "deleting destructor through the voice pool: Drop covers it, the pool needs its own state struct",
        ],
    },
    Row {
        channel: "DSound",
        method: "vf5",
        state: State::Missing,
        narrows: &["a single forwarded device call: no behaviour in it"],
    },
    Row {
        channel: "DSound",
        method: "vf1",
        state: State::Missing,
        narrows: &[
            "buffer attach through device, allocator and mixer globals: needs those subsystem traits",
        ],
    },
    Row {
        channel: "DSound",
        method: "vf11",
        state: State::Missing,
        narrows: &[
            "refill hook with device lock/unlock, globals and 3D math: needs the device trait and audio state",
        ],
    },
    // ADPCM DirectSound voice: four lifted, one missing.
    Row {
        channel: "DSoundAdpcm",
        method: "vf7",
        state: State::Proven,
        narrows: &["1/0 answer narrows to bool", "cursor must select a lane"],
    },
    Row {
        channel: "DSoundAdpcm",
        method: "vf8",
        state: State::Proven,
        narrows: &[
            "source words arrive as a slice",
            "cursor must select a lane; divisor must be nonzero",
            "resolved table entries must stay in the owned tables",
            "the cursor is read once: collaborators borrow the world, never the voice",
        ],
    },
    Row {
        channel: "DSoundAdpcm",
        method: "vf10",
        state: State::Proven,
        narrows: &["the play cursor travels by value from the device call"],
    },
    Row {
        channel: "DSoundAdpcm",
        method: "vf3",
        state: State::Proven,
        narrows: &[
            "the gain word travels as owned data",
            "the overwritten incoming-argument slot is unmodelled: it feeds only the published cursor, which is compared",
        ],
    },
    Row {
        channel: "DSoundAdpcm",
        method: "vf0",
        state: State::Missing,
        narrows: &[
            "deleting destructor through the voice pool: Drop covers it, the pool needs its own state struct",
        ],
    },
];

/// Number of rows in each state.
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
