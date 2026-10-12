//! Registry of the follow-point family: one row per method of the class,
//! with its state and what each proof narrows.
//!
//! State values:
//! - `Proven`: lifted here and proven against its verified 32-bit rewrite
//!   by the differential crate `lf-peds-follow-diff`.
//! - `Unlifted`: verified, not lifted yet; the reason is in `note`.
//!
//! Method names are the class's own names; no addresses appear here.

/// How far a method has been lifted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// Lifted here and proven against its verified rewrite.
    Proven,
    /// Verified, but not lifted yet.
    Unlifted,
}

/// One method of the family.
#[derive(Clone, Copy, Debug)]
pub struct Method {
    /// The method's name in the class.
    pub name: &'static str,
    /// Where it stands.
    pub state: State,
    /// What the proof narrows, or why the method is not lifted.
    pub note: &'static str,
}

/// Every verified method of the family, in the order of the class's
/// virtual table.
pub const METHODS: &[Method] = &[
    Method {
        name: "vf7 (current point copy)",
        state: State::Proven,
        note: "Out-pointer answer narrowed to the returned point. Proven for cursors inside the current route; the original faults on a null route, so the lift panics there.",
    },
    Method {
        name: "vf12 (handle copy)",
        state: State::Proven,
        note: "Output pointer answer narrowed: the record is the argument and the result is not returned. The word is read as the handle, as the rewrite's own comment says.",
    },
    Method {
        name: "vf18 (advance after sub-task)",
        state: State::Proven,
        note: "The sub-task type is queried once in the lift and up to four times in the rewrite; the query has no effect, so the call log compares its answers, not its count.",
    },
    Method {
        name: "vf19 (pick waypoint)",
        state: State::Proven,
        note: "The ped is read through a snapshot taken by the caller. The route search covers every segment and the nearest-point fallback; not-a-number payloads are pinned by operand order.",
    },
    Method {
        name: "vf21 (waypoint copy)",
        state: State::Proven,
        note: "Output pointer answer narrowed to the returned point. Proven for indices inside the waypoint count; the lift panics past it.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::CTaskComplexMoveFollowPointRoute",
        state: State::Unlifted,
        note: "Writes the class's vtable words, calls the move-base constructor and the route constructor (which build the route table), and reads back a flag word after the base call. Needs the callee effects modelled and the route table built from the constructor's arguments.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::CTaskComplexMoveFollowPointRoute_2",
        state: State::Unlifted,
        note: "Same as the first constructor, with the speed word passed in rather than selected through a helper. Waits on the same constructor model.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::vf1 (clone)",
        state: State::Unlifted,
        note: "Calls the thread allocator and the constructor with fields of the source. Waits on the constructor lift.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::~CTaskComplexMoveFollowPointRoute",
        state: State::Unlifted,
        note: "Writes vtable words and releases an owned field through a shared context object and the chained destructor. Needs the global context and the two callees modelled.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::vf5 (poll)",
        state: State::Unlifted,
        note: "Calls about ten virtual slots on the owner and the sub-task, one direct helper and the owner's finish call. The owner is the same word as the sub-task slot in the verified rewrites; not modelled yet.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::dtor_thunk",
        state: State::Unlifted,
        note: "Not verified (the lane list marks it unverified), so not lifted.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::control_sub_task",
        state: State::Unlifted,
        note: "Not verified yet.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::create_route_leg_subtask",
        state: State::Unlifted,
        note: "Not verified yet.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::get_arrival_radius",
        state: State::Unlifted,
        note: "Not verified yet.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::get_task_id",
        state: State::Unlifted,
        note: "Not verified yet.",
    },
];

/// How many methods are proven.
#[must_use]
pub fn proven_count() -> usize {
    METHODS.iter().filter(|m| m.state == State::Proven).count()
}
