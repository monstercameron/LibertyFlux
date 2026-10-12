//! What is lifted for the move-between-points scenario, what is proven, and what
//! each proof narrows.
//!
//! Each entry is proven by the differential test crate `lf-peds-diff`, which runs
//! the lifted method and the verified 32-bit rewrite on the same generated inputs
//! and compares the results, the calls in order with their arguments, and the
//! words each side writes. The rewrites are included unchanged.

/// One method of the family and its status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    /// The method's name in the class.
    pub method: &'static str,
    /// Lifted and proven, or left out.
    pub status: Status,
    /// What the proof covers and what it does not.
    pub note: &'static str,
}

/// Whether a method is in the lifted module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Written in the module and proven against its verified rewrite.
    LiftedAndProven,
    /// Verified by the checker, but not lifted in this pass.
    VerifiedNotLifted,
    /// Not verified by the checker; not written.
    Unverified,
}

/// The methods of the scenario class, in virtual-table order (the order is
/// recorded here for reference only; the lifted code has no slots).
pub const METHODS: &[Entry] = &[
    Entry {
        method: "deleting_destructor",
        status: Status::LiftedAndProven,
        note: "Calls the base class destructor and, for bit 0 of the flags, the heap free; \
               both are host calls, compared in order with their arguments. The rewrite reads \
               the heap word after the destructor. The proof covers flags with bit 0 set and \
               clear. The lift writes no memory: the owner drops the value.",
    },
    Entry {
        method: "clone_scenario",
        status: Status::LiftedAndProven,
        note: "Allocation through the heap, then construction from three words of the source. \
               The proof covers allocation failure (zero) and success with a range of aux words \
               and constructor results. Call order and arguments are compared. The constructed \
               object is a host effect, so it is compared as a call, not as memory.",
    },
    Entry {
        method: "blend_weight",
        status: Status::LiftedAndProven,
        note: "Pure data: status bit 1, activity byte, guard word and state word. The proof \
               covers every combination of status bit 1, activity, guard zero or not, and states \
               0, 1, 2 and two extremes. The rewrite's header calls the state word a waypoint \
               count; the lift calls it the state, as the other methods do.",
    },
    Entry {
        method: "run_tick",
        status: Status::LiftedAndProven,
        note: "Runs the sub-task unless its latch bit 0 is set, sets latch bit 1 on success, \
               resets the state from one and releases a non-zero payload. The proof covers latch \
               values, run answers with zero and non-zero low bytes, states, payload present or \
               absent, and distinct run arguments. The sub-task is passed in by the caller (the \
               rewrite reads it from the scenario), so a null sub-task is not expressible here: \
               the original faults on one. The rewrite's header names a different class for this \
               method (a simple scenario); the field meanings it gives for the state and payload \
               words differ from the other methods of this class. Whether the method is inherited \
               from that base is unknown.",
    },
    Entry {
        method: "create_first_sub_task",
        status: Status::Unverified,
        note: "Named by a naming lane; no verified rewrite. Not written.",
    },
    Entry {
        method: "create_next_sub_task",
        status: Status::Unverified,
        note: "Named by a naming lane; no verified rewrite. Not written.",
    },
    Entry {
        method: "tick (the main scenario update)",
        status: Status::VerifiedNotLifted,
        note: "Verified rewrite exists (about 1,300 bytes, 368 instructions, 46 blocks). Not \
               lifted in this pass: it calls about 25 callees and collaborators, and its heading \
               block takes a double-precision answer that the rewrite maps back from three \
               scripted low words (see the proof gaps in the report). The rewrite maps any other \
               answer to zero, so a lift cannot be proven against it beyond those three values.",
    },
];
