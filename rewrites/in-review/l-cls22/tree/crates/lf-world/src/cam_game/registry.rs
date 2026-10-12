//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the game camera class, plus the
//! methods of that class that have no verified rewrite yet. `Proven` means
//! the method is restated on [`CCamGame`](super::CCamGame) and the
//! `lf-camgame-diff` test crate ran it against its verified rewrite on the
//! same generated inputs, comparing the result, every field the rewrite
//! may change, and every collaborator call in order, with a deliberately
//! wrong lift caught alongside.

/// Lift state of one method of the class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on the camera type and proven against its rewrite.
    Proven,
    /// Verified, but not lifted, for the stated reason.
    Missing,
}

/// One method's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Slot name in the 32-bit form, e.g. `"vf6"`.
    pub method: &'static str,
    /// What the method does, in the lifted form's name.
    pub lifted_as: &'static str,
    /// Lift state.
    pub state: State,
    /// Calls the lifted method makes on the world, in order. Empty when
    /// the method calls nothing.
    pub calls: &'static [&'static str],
    /// What the lift narrows away, or why the method is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified method of the class and the two methods without a
/// rewrite that the class lists.
pub const ROWS: &[Row] = &[
    Row {
        method: "vf0",
        lifted_as: "CCamGame::deleting_destructor",
        state: State::Proven,
        calls: &["destroy_body", "pool_manager", "pool_release"],
        narrows: &[
            "the return value (the object's own identity) is not returned",
            "the pool manager is read through the world, not from a global",
            "the destructor body is an unlifted callee; its effect is the world's",
        ],
    },
    Row {
        method: "vf1",
        lifted_as: "CCamGame::restart",
        state: State::Proven,
        calls: &[
            "teardown",
            "create_worker",
            "chain_set_flags",
            "release_worker",
        ],
        narrows: &[
            "the constant answer 1 is not returned",
            "the worker is never zero in the proof; the original reads through it",
            "the teardown is the object's dispatched slot 6: the lift does not read the table",
            "the worker's release is the worker's own dispatched slot 1",
        ],
    },
    Row {
        method: "vf6",
        lifted_as: "CCamGame::reset_to_idle",
        state: State::Proven,
        calls: &["shared_default_word", "reset_state_block"],
        narrows: &[
            "the constant answer 1 is not returned",
            "the reset step is an unlifted callee that may change any field",
        ],
    },
    Row {
        method: "vf4",
        lifted_as: "CCamGame::gate_update",
        state: State::Proven,
        calls: &[
            "inhibit_flag",
            "gate_status",
            "gate_byte",
            "find_target",
            "gate_word",
            "notify_sink",
            "sync_target",
            "singleton_present",
            "middle_update",
            "tail",
            "tail_state",
            "tail_flags",
            "set_tail_flags",
            "set_tail_state",
            "set_matched",
            "clear_done",
        ],
        narrows: &[
            "the constant answer 1 is not returned",
            "the sub-object address (the camera plus a fixed offset) is computed by the host",
            "the gate and tail objects are read and written through the world; their layout is the host's",
            "the watcher's selector and the sink's two flags are constants of the call sites",
        ],
    },
    Row {
        method: "vf5",
        lifted_as: "",
        state: State::Missing,
        calls: &[],
        narrows: &[
            "verified, not lifted: eleven callees, a large body and many globals",
        ],
    },
    Row {
        method: "set_hint_move_in_distance_default",
        lifted_as: "",
        state: State::Missing,
        calls: &[],
        narrows: &[
            "not decoded: the inventory records no size for it; no rewrite",
        ],
    },
    Row {
        method: "set_hint_times_default",
        lifted_as: "",
        state: State::Missing,
        calls: &[],
        narrows: &[
            "not decoded: the inventory records no size for it; no rewrite",
        ],
    },
];

/// Number of rows that are proven.
#[must_use]
pub fn proven_count() -> usize {
    ROWS.iter().filter(|r| r.state == State::Proven).count()
}
