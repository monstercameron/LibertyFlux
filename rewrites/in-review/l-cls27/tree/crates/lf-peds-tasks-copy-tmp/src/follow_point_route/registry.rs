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
        note: "The ped is read through a snapshot the caller takes. Covers the search over every segment and the nearest-point fallback.",
    },
    Method {
        name: "vf21 (waypoint copy)",
        state: State::Proven,
        note: "Output pointer answer narrowed to the returned point. Proven for indices inside the waypoint count; the lift panics past it.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::CTaskComplexMoveFollowPointRoute",
        state: State::Proven,
        note: "Selector form: the speed helper runs first, then the move base, then the route constructor, as the rewrite does. The move base's effect on the flag word is a host call and the flag word is read back after it. The words kept for copying (speed, target, route arguments, final value) reach the proof through the call arguments; their stores into memory are inside the callees and are not compared. Vtable words are not represented.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::CTaskComplexMoveFollowPointRoute_2",
        state: State::Proven,
        note: "Speed given as an argument; otherwise as the selector form. Same narrowings. The route table is stored as the current route, and the proof compares only whether the word is null, not which table it names.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::vf1 (clone)",
        state: State::Proven,
        note: "The allocation is a host call (the allocator global is read on the host side). The new task is constructed through the lifted constructor; the proof runs it against the verified constructor rewrite behind the callee. A null allocation returns no task. The copied words are compared through the constructor's call arguments.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::~CTaskComplexMoveFollowPointRoute",
        state: State::Proven,
        note: "Releases the owned route word through the host when it is not zero, then runs the move base's destructor and returns its answer. The host reads the release context. Vtable words are not represented; the proof checks them against the constants the rewrite installs.",
    },
    Method {
        name: "CTaskComplexMoveFollowPointRoute::vf5 (poll)",
        state: State::Proven,
        note: "Owner, sub-task, slot and helper calls and reads are host methods. The scratch words passed to the slot and int calls are not compared: the lift does not pass them and the proof's stubs answer from their own buffer. The route count is cleared when the second argument is null, which drops the entries past the count. The word at offset 0x08 is the owner here and the current sub-task in the advance method; see the notes.",
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
