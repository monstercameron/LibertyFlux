//! What each lifted method covers, and what it narrows.
//!
//! One row per verified method of the mirror-reflection render phase. A row
//! says whether the method is lifted and proven, names the virtual slot it
//! answers, and lists what the lift narrows away from the verified rewrite.
//! The counts the progress reports use come from this table, and a host test
//! pins them.

/// The state of one method in this lift.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiftState {
    /// Lifted, and the differential proof against its verified rewrite runs.
    Proven,
    /// Verified in the original, not lifted yet.
    Unlifted,
}

/// One verified method of the render phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MethodRecord {
    /// The method's name in this module.
    pub name: &'static str,
    /// The virtual slot of the phase object the method answers.
    pub slot: u32,
    /// Whether the method is lifted, and proven.
    pub state: LiftState,
    /// What the lift narrows, one line each.
    pub narrowed: &'static [&'static str],
}

/// The verified methods of the render phase, in slot order.
pub const METHODS: &[MethodRecord] = &[
    MethodRecord {
        name: "MirrorReflectionPhase::deleting_destructor",
        slot: 0,
        state: LiftState::Proven,
        narrowed: &[
            "returns nothing; the original returns the object it was given, which the caller already holds",
            "the destructor and the storage release are the host's; the lift makes no other call",
        ],
    },
    MethodRecord {
        name: "MirrorReflectionPhase::refresh",
        slot: 3,
        state: LiftState::Proven,
        narrowed: &[
            "the parameter block must be non-null on the gated path; the original faults on a null one",
            "the early returns answer zero; the original answers its accumulator there. The inventory lists no direct caller (reached only through the virtual table), so the answer is unchecked, not read",
            "the target and parameter-block handle are re-read after the rebuild, which is given the phase; a collaborator not given the phase or one of its members is assumed not to write those words",
            "the remembered word is named as the identity with its low byte replaced, not computed from the address",
            "the parameter table is a slice; an entry outside it panics rather than reading the image; the proof draws swap indices 0 to 20 of a 64-word table",
        ],
    },
    MethodRecord {
        name: "MirrorReflectionPhase::reflect",
        slot: 4,
        state: LiftState::Proven,
        narrowed: &[
            "returns the status bit only; the original returns the snapshot pointer with its low byte replaced",
            "the snapshot's selector and status are read through the host at the points the original reads them",
            "the target and parameter-block handle are re-read after the binds, which are given a member of the phase; a call given no member of the phase is assumed not to write those words",
            "the verified rewrite's contract sets the unused-stack fill to zero; this lift does not change that",
            "the scaled matrix's multiply pins both operands (black_box); the rewrite's own operand order is compared, not assumed",
        ],
    },
    MethodRecord {
        name: "MirrorReflectionPhase::render",
        slot: 8,
        state: LiftState::Proven,
        narrowed: &[
            "returns nothing; the original returns zero",
            "the target is re-read after the probe, which is given a member of the phase; a call given no member of the phase is assumed not to write it",
            "the staged float is chosen by the probe's low byte; its high bytes are compared as values",
        ],
    },
];

/// The number of methods lifted and proven.
#[must_use]
pub fn proven_count() -> usize {
    METHODS
        .iter()
        .filter(|m| m.state == LiftState::Proven)
        .count()
}
