//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the player pedestrian class that
//! has behaviour in it, and one row for each verified method that is left
//! out, with the reason. `Proven` means the method is restated on
//! [`CPlayerPed`](super::CPlayerPed) and the `lf-playerped-diff` test crate
//! ran it against its verified rewrite on the same generated inputs,
//! comparing the result, every byte of the pedestrian's image the rewrite
//! may change, and every collaborator call in order, with a deliberately
//! wrong lift caught alongside.

/// Lift state of one verified method of the class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on the pedestrian type and proven against its rewrite.
    Proven,
    /// Verified, but not lifted, for the stated reason.
    NotLifted,
}

/// One method's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Slot name in the 32-bit form, e.g. `"vf62"`, or the method's name.
    pub method: &'static str,
    /// What the method does, in the lifted form's name.
    pub lifted_as: &'static str,
    /// Lift state.
    pub state: State,
    /// Calls the lifted method makes on the world, in order. Empty when the
    /// method calls nothing.
    pub calls: &'static [&'static str],
    /// What the lift narrows away, or why the method is not lifted. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified method of the class: the nine proven rows, then the
/// verified methods that are not lifted yet.
pub const ROWS: &[Row] = &[
    Row {
        method: "vf62",
        lifted_as: "CPlayerPed::add_motion_value",
        state: State::Proven,
        calls: &["motion_helper"],
        narrows: &[
            "the helper's answer is returned as the method's answer; the rewrite does the same",
        ],
    },
    Row {
        method: "vf44",
        lifted_as: "CPlayerPed::notify_while_linked",
        state: State::Proven,
        calls: &["trigger_entry", "base_update"],
        narrows: &[
            "the base update is a tail forward in the original; the lift calls it and returns its answer",
        ],
    },
    Row {
        method: "vf16",
        lifted_as: "CPlayerPed::run_jobs_if_ready",
        state: State::Proven,
        calls: &["ped_ready_check", "ped_job_a", "ped_job_b"],
        narrows: &[
            "the rewrite's result is 0 or 1 in full; the lift returns a bool and the test widens it",
        ],
    },
    Row {
        method: "vf69",
        lifted_as: "CPlayerPed::reset_and_apply",
        state: State::Proven,
        calls: &["pre_step", "apply_step", "notify_object"],
        narrows: &[
            "the mode nibble is tested over the half-word at the mode offset; the rest of that half-word is not read",
        ],
    },
    Row {
        method: "vf19",
        lifted_as: "CPlayerPed::notify_sub_objects",
        state: State::Proven,
        calls: &["dispatch_slot8"],
        narrows: &[
            "the virtual dispatch is one host method: the proof checks that the rewrite calls slot 8 of each sub-object, with the other slots set to a trap that would be recorded",
            "the sub-objects are test-owned objects whose table holds the trap and the slot-8 stub only",
        ],
    },
    Row {
        method: "vf47",
        lifted_as: "CPlayerPed::setup_and_store",
        state: State::Proven,
        calls: &["player_context", "lookup_context", "setup", "finalize"],
        narrows: &[
            "the context global is read through the world; the proof sets it as a scripted word, zero included",
            "the read back is the pedestrian's own stored word after the follow-up helper",
        ],
    },
    Row {
        method: "change_model",
        lifted_as: "CPlayerPed::change_model",
        state: State::Proven,
        calls: &["model_state", "lookup_model", "preload_model", "revive", "load_bundle"],
        narrows: &[
            "the model table is read through the world; the proof fills eight entries, each pointing at a test-owned record",
            "a table entry that is zero is a fault in the original and is not generated: the proof covers model numbers 0 to 7 only",
        ],
    },
    Row {
        method: "vf80",
        lifted_as: "CPlayerPed::set_transform",
        state: State::Proven,
        calls: &[],
        narrows: &[
            "the source block is passed by value; an overlap of the source with the transform field is not generated",
        ],
    },
    Row {
        method: "vf79",
        lifted_as: "CPlayerPed::set_descriptor",
        state: State::Proven,
        calls: &[],
        narrows: &[
            "the original's zero in its result register is not compared: the method has no result",
            "the source block is passed by value; an overlap with the descriptor field is not generated",
        ],
    },
    Row {
        method: "vf78",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &["a single store of one word and a return of it: no behaviour in the method"],
    },
    Row {
        method: "vf81",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &["a single store of one word: no behaviour in the method"],
    },
    Row {
        method: "vf60",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &[
            "verified, but a store of one float word followed by one forwarded call: no behaviour beyond the forward; not lifted",
        ],
    },
    Row {
        method: "vf32",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &[
            "verified: three calls on a sub-object that the pedestrian reaches through a pointer field; not lifted in this pass",
        ],
    },
    Row {
        method: "vf45",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &[
            "verified: a squared distance in single-precision scalar operations over two sub-objects and a global threshold; not lifted in this pass",
        ],
    },
    Row {
        method: "acquire_aim_target",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &[
            "verified: the not-a-number selection is pinned by hand and it reads a global state word; not lifted in this pass",
        ],
    },
    Row {
        method: "vf29",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &["verified, 1613 bytes of the original: not lifted in this pass"],
    },
    Row {
        method: "vf31",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &["verified, 861 bytes of the original: not lifted in this pass"],
    },
    Row {
        method: "vf33",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &["verified, 364 bytes of the original: not lifted in this pass"],
    },
    Row {
        method: "vf39",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &["verified, 289 bytes of the original: not lifted in this pass"],
    },
    Row {
        method: "vf54",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &["verified, 305 bytes of the original: not lifted in this pass"],
    },
    Row {
        method: "vf73",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &["verified, 488 bytes of the original: not lifted in this pass"],
    },
];

/// Number of rows that are proven.
#[must_use]
pub fn proven_count() -> usize {
    ROWS.iter().filter(|r| r.state == State::Proven).count()
}

/// Number of verified rows that are not lifted.
#[must_use]
pub fn not_lifted_count() -> usize {
    ROWS.iter().filter(|r| r.state == State::NotLifted).count()
}
