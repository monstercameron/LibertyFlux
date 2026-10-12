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
        calls: &[
            "model_state",
            "lookup_model",
            "preload_model",
            "revive",
            "load_bundle",
        ],
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
        lifted_as: "CPlayerPed::run_entry_chain",
        state: State::Proven,
        calls: &["chain_reset", "chain_flag", "chain_main"],
        narrows: &[
            "the sub-object is an address the chain computes and passes on; the sub-object's own entries are the world's and are not run",
            "the chain base is read once; a change the first entry makes to it is not seen by the original either, since the original reads it once",
        ],
    },
    Row {
        method: "vf45",
        lifted_as: "CPlayerPed::relink_on_move",
        state: State::Proven,
        calls: &[
            "base_entry",
            "read_word",
            "distance_threshold_bits",
            "relink",
        ],
        narrows: &[
            "the point components are read through the world's word read: the lift computes the addresses, the proof compares the results and the block and flag writes, not the reads themselves",
            "the squared distance is pinned operand by operand with black_box; the original's own sums are not pinned, and the proof did not show a difference",
            "the source and target points are kept records the case builds; the target is absent or the kept point, since the original dereferences it",
        ],
    },
    Row {
        method: "acquire_aim_target",
        lifted_as: "CPlayerPed::acquire_aim_target",
        state: State::Proven,
        calls: &[
            "get_context",
            "context_property_argument",
            "get_property",
            "property_threshold_bits",
            "aim_state",
            "state_gate",
            "aim_factor_bits",
            "create_target",
            "attach_target",
            "mark_selected",
        ],
        narrows: &[
            "the context, property and aim records are read and written through the world; the proof builds them as test-owned records and both sides read the same bytes",
            "the rewrite's result is 0 or 1 in full; the lift returns a bool and the test widens it",
            "the state gate's answer is compared in its low byte, as the original tests it",
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
        lifted_as: "CPlayerPed::refresh_gate",
        state: State::Proven,
        calls: &[
            "hook_argument",
            "hook_call",
            "global_byte",
            "global_word",
            "refresh_virtual",
            "refresh_follow",
            "pose_entry",
            "entry_flag",
            "read_word",
            "attach_virtual",
            "probe_first",
            "probe_second",
            "deep_refresh_gate",
            "deep_refresh_entry",
            "sub_virtual",
            "ready_probe",
            "tail_virtual",
        ],
        narrows: &[
            "the globals are read through the world; the proof sets each as a cell of the runtime, zero and nonzero included",
            "the virtual slots (refresh, attach, sub-object, tail) are the host's dispatch: the proof checks the rewrite calls the right slot by trapping the other slots",
            "the pose table is filled for indexes minus eight to seven; the rewrite reads it through a signed index, and no index outside that range is generated",
            "the hook is the stdcall target a global slot holds; the proof installs one hook stub",
        ],
    },
    Row {
        method: "vf39",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &[
            "not lifted: the verified rewrite stores zero in the fourth output word, which the original leaves to an uninitialised stack slot; the rewrite's proof depends on the checker's zero stack fill, so a lift would fix a value the original does not define",
        ],
    },
    Row {
        method: "vf54",
        lifted_as: "",
        state: State::NotLifted,
        calls: &[],
        narrows: &[
            "not lifted: the verified rewrite stores zero in the fourth output word, which the original reads from an uninitialised stack scratch slot; the proof depends on the checker's zero stack fill, so a lift would fix a value the original does not define",
        ],
    },
    Row {
        method: "vf73",
        lifted_as: "CPlayerPed::settle_budget",
        state: State::Proven,
        calls: &[
            "base_step",
            "current_item",
            "classify_task",
            "task_present",
            "award",
            "frame_delta_bits",
            "cost_scale_bits",
            "heat_limit",
            "read_byte",
        ],
        narrows: &[
            "the classifier writes two words through pointer arguments; the proof answers them as scripted values and logs the pointer arguments as a shape marker, not as addresses",
            "the classifier's tag byte is written but never read by the step, so the proof does not compare it",
            "the task chain is walked through the world's word read; the proof generates chains of up to three nodes that end; a cyclic chain would not end in the original either",
            "the frame cost is a float product of two globals; not-a-number and values beyond the 64-bit range take the integer indefinite, as the original does, and the proof generates those values",
        ],
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
