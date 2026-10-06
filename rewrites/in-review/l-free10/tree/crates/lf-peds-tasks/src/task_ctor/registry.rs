//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified `task_ctor_*` free function on the lane list,
//! grouped by the data shape the functions share. `Proven` means the
//! routine is restated on its owning type and the differential test
//! crate ran it against its verified rewrite on the same generated
//! inputs, comparing results and every effect, with a deliberately
//! wrong lift caught alongside. Counts below come from this table.

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
    /// Owning data shape, e.g. `"ParamBlock"`.
    pub shape: &'static str,
    /// Verified name, e.g. `"task_ctor_36_vec_bytes"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the routine is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every listed routine, grouped by data shape.
pub const ROWS: &[Row] = &[
    // The task parameter block: seven base+main kinds proven.
    Row {
        shape: "ParamBlock",
        method: "task_ctor_3e_single_float",
        state: State::Proven,
        narrows: &[
            "the base and main collaborators travel as the TaskInit trait: the proof plants stubs and compares slot and arguments in order",
            "the shared word the header forwards travels as the argument g, mapped onto its relocated word by the proof",
            "arguments the rewrite uses as bytes arrive as u8: the proof feeds the rewrite full words with random high bytes and the lift their low bytes",
            "the rewrite's constant 0 answer is asserted by the proof, not modelled",
        ],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_36_vec_bytes",
        state: State::Proven,
        narrows: &[
            "the base and main collaborators travel as the TaskInit trait: the proof plants stubs and compares slot and arguments in order",
            "the shared word the header forwards travels as the argument g, mapped onto its relocated word by the proof",
            "the rewrite's constant 0 answer is asserted by the proof, not modelled",
        ],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_34_float_pair",
        state: State::Proven,
        narrows: &[
            "the base and main collaborators travel as the TaskInit trait: the proof plants stubs and compares slot and arguments in order",
            "the shared word the header forwards travels as the argument g, mapped onto its relocated word by the proof",
            "the two reads of the folded flag byte collapse to one: no write to that byte sits between them",
            "arguments the rewrite uses as bytes arrive as u8: the proof feeds the rewrite full words with random high bytes and the lift their low bytes",
            "the rewrite's constant 0 answer is asserted by the proof, not modelled",
        ],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_3f_toggle",
        state: State::Proven,
        narrows: &[
            "the base and main collaborators travel as the TaskInit trait: the proof plants stubs and compares slot and arguments in order",
            "the shared word the header forwards travels as the argument g, mapped onto its relocated word by the proof",
            "the two reads of the flag byte collapse to one: no write to that byte sits between them",
            "arguments the rewrite uses as bytes arrive as u8: the proof feeds the rewrite full words with random high bytes and the lift their low bytes",
            "the rewrite's constant 0 answer is asserted by the proof, not modelled",
        ],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_40_toggle_float",
        state: State::Proven,
        narrows: &[
            "the base, main and word-quantiser collaborators travel as the TaskInit trait: the proof plants stubs and compares slot and arguments in order",
            "the shared word the header forwards travels as the argument g, mapped onto its relocated word by the proof",
            "the two reads of the toggled byte collapse to one: no write to that byte sits between them",
            "arguments the rewrite uses as bytes arrive as u8: the proof feeds the rewrite full words with random high bytes and the lift their low bytes",
            "the rewrite's constant 0 answer is asserted by the proof, not modelled",
        ],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_2f_vec5_flags",
        state: State::Proven,
        narrows: &[
            "the base, main and block collaborators travel as the TaskInit trait: the proof plants stubs and compares slot and arguments in order",
            "the shared word the header forwards travels as the argument g, mapped onto its relocated word by the proof",
            "arguments the rewrite uses as bytes arrive as u8: the proof feeds the rewrite full words with random high bytes and the lift their low bytes",
            "the rewrite's constant 0 answer is asserted by the proof, not modelled",
        ],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_43_chain",
        state: State::Proven,
        narrows: &[
            "the base, main and two chain-tail collaborators travel as the TaskInit trait: the proof plants stubs and compares slot and arguments in order",
            "the shared word the header forwards travels as the argument g, mapped onto its relocated word by the proof",
            "the rewrite's constant 0 answer is asserted by the proof, not modelled",
        ],
    },
    // The parameter block's main-only and 0x3e-derived kinds: same
    // block, distinct tails; the next sub-groups.
    Row {
        shape: "ParamBlock",
        method: "task_ctor_3d_minimal",
        state: State::Missing,
        narrows: &["main initialiser plus tag byte only: next sub-group"],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_42_chain",
        state: State::Missing,
        narrows: &["main plus one tail call plus tag byte: next sub-group"],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_46_single_float",
        state: State::Missing,
        narrows: &["main, one tail call, one word store, tag byte: next sub-group"],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_35_flags_packed",
        state: State::Missing,
        narrows: &[
            "main, block copy, five-argument flag packing over the old flag byte: next sub-group",
        ],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_33_quantize",
        state: State::Missing,
        narrows: &["main, block copy, vector-quantiser call, flag fold: next sub-group"],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_41_flags_float",
        state: State::Missing,
        narrows: &["main, block copy, byte-quantiser call, flag packing, one word: next sub-group"],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_44_trunc_flag",
        state: State::Missing,
        narrows: &[
            "main, two tail calls, float-to-int truncation folded with a flag bit: next sub-group",
        ],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_45_branch_flags",
        state: State::Missing,
        narrows: &["main, a branch choosing one tail call or two, two flag folds: next sub-group"],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_3a_sub_flags",
        state: State::Missing,
        narrows: &[
            "chains the kind-0x3e initialiser, stamps its kind byte, packs flags: third sub-group",
        ],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_3b_compare_consts",
        state: State::Missing,
        narrows: &[
            "chains the kind-0x3e initialiser, stamps its kind byte, sets a flag bit on float-constant equality: third sub-group",
        ],
    },
    Row {
        shape: "ParamBlock",
        method: "task_ctor_3c_copy3",
        state: State::Missing,
        narrows: &[
            "chains the kind-0x3e initialiser, stamps its kind byte, copies three words: third sub-group",
        ],
    },
    // The small vtable'd task objects: vtable stamp plus flag or bound
    // handle; the register/retain call needs its trait design.
    Row {
        shape: "TaskObjSmall",
        method: "task_ctor_flag_u8",
        state: State::Missing,
        narrows: &[
            "base constructor, flag byte, vtable stamp, two state words: needs the small-task owning type",
        ],
    },
    Row {
        shape: "TaskObjSmall",
        method: "task_ctor_arg_bind",
        state: State::Missing,
        narrows: &[
            "six-argument member initialiser, vtable stamp, bound-argument register call: needs the small-task owning type",
        ],
    },
    Row {
        shape: "TaskObjSmall",
        method: "task_ctor_vec_copy",
        state: State::Missing,
        narrows: &[
            "base constructor, handle store, vtable stamp, three-word copy, two register calls: needs the small-task owning type",
        ],
    },
    // The guarded follow-up builders: state dispatch through switch
    // tables, RNG-weighted durations and multi-gate branches.
    Row {
        shape: "TaskBuilder",
        method: "task_ctor_random_duration",
        state: State::Missing,
        narrows: &[
            "random-duration task through creator and qualifier calls with float tuning: needs the builder-trait design",
        ],
    },
    Row {
        shape: "TaskBuilder",
        method: "task_ctor_state_dispatch",
        state: State::Missing,
        narrows: &[
            "state-number dispatch through a jump table with host and subtype reads: needs the builder-trait design",
        ],
    },
    Row {
        shape: "TaskBuilder",
        method: "task_ctor_guarded_branch",
        state: State::Missing,
        narrows: &[
            "two dispatch switches behind gate words, 892 bytes: needs the builder-trait design",
        ],
    },
    // The RNG-derived task: game-RNG state and float scale globals.
    Row {
        shape: "RngTask",
        method: "task_ctor_rng_floats",
        state: State::Missing,
        narrows: &[
            "flag fold, vtable stamp, two multiply-add RNG draws scaled into float fields: needs the RNG-state design",
        ],
    },
    // The point-carrying task objects: one or two vtable stamps, copied
    // points, scalar stores, retain calls, a global time stamp.
    Row {
        shape: "TaskObjPoint",
        method: "task_ctor_simple",
        state: State::Missing,
        narrows: &["base chain plus one vtable stamp: needs the point-task owning types"],
    },
    Row {
        shape: "TaskObjPoint",
        method: "task_ctor_tagged_float",
        state: State::Missing,
        narrows: &["base chain plus two vtable stamps: needs the point-task owning types"],
    },
    Row {
        shape: "TaskObjPoint",
        method: "task_ctor_xyz_float",
        state: State::Missing,
        narrows: &[
            "two vtable stamps, one copied point, one word: needs the point-task owning types",
        ],
    },
    Row {
        shape: "TaskObjPoint",
        method: "task_ctor_xyz_pair",
        state: State::Missing,
        narrows: &[
            "two vtable stamps, one point copied twice, mixed-width zeroing with a two-byte gap: needs the point-task owning types",
        ],
    },
    Row {
        shape: "TaskObjPoint",
        method: "task_ctor_mid",
        state: State::Missing,
        narrows: &[
            "one vtable stamp, one point, scalars, retain call, time stamp, span constant: needs the point-task owning types",
        ],
    },
    Row {
        shape: "TaskObjPoint",
        method: "task_ctor_full_a",
        state: State::Missing,
        narrows: &[
            "same body as task_ctor_full_b with another vtable (one generic method, two instances): needs the point-task owning types",
        ],
    },
    Row {
        shape: "TaskObjPoint",
        method: "task_ctor_full_b",
        state: State::Missing,
        narrows: &[
            "same body as task_ctor_full_a with another vtable (one generic method, two instances): needs the point-task owning types",
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
