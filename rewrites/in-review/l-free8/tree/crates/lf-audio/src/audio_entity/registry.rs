//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified `audio_entity_*` routine in the lane's list.
//! `Proven` means the routine is restated on its owning type and the
//! differential test crate ran it against its verified rewrite on the
//! same generated inputs, comparing results and every effect, with a
//! deliberately wrong lift caught alongside. Counts below come from
//! this table.

/// Lift state of one verified routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its owning type and proven against its rewrite.
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
    /// Owning structure in the lift, e.g. `"record"`.
    pub group: &'static str,
    /// Routine name in the 32-bit form, e.g. `"audible_check"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the routine is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified routine of the group, in structure order.
pub const ROWS: &[Row] = &[
    // Record gates: two lifted and proven.
    Row {
        group: "record",
        method: "audible_check",
        state: State::Proven,
        narrows: &[
            "the 0/1 answer narrows to a bool; the five playback globals arrive as one argument",
        ],
    },
    Row {
        group: "record",
        method: "active_check",
        state: State::Proven,
        narrows: &[
            "the 0/1 answer narrows to a bool; the five playback globals arrive as one argument",
        ],
    },
    // Event table: one lifted and proven.
    Row {
        group: "events",
        method: "event_append",
        state: State::Proven,
        narrows: &[
            "the always-zero answer narrows to appended/not; the key must select an owned row, else panic; over-full counts narrow to full",
        ],
    },
    // Lifecycle: two lifted and proven.
    Row {
        group: "lifecycle",
        method: "construct",
        state: State::Proven,
        narrows: &[
            "the two installed table addresses are pinned on the 32-bit side only; member addresses narrow to construction order",
        ],
    },
    Row {
        group: "lifecycle",
        method: "latch_config",
        state: State::Proven,
        narrows: &[
            "the flag byte narrows to a bool (any nonzero reads as latched); the meaningless answer is unmodelled",
        ],
    },
    // Static init stubs: forwarder-only, never lifted.
    Row {
        group: "init",
        method: "init_1",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    Row {
        group: "init",
        method: "init_2",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    Row {
        group: "init",
        method: "init_3",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    Row {
        group: "init",
        method: "init_4",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    Row {
        group: "init",
        method: "init_5",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    Row {
        group: "init",
        method: "init_6",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    Row {
        group: "init",
        method: "init_7",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    Row {
        group: "init",
        method: "init_8",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    Row {
        group: "init",
        method: "init_9",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    Row {
        group: "init",
        method: "init_10",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    Row {
        group: "init",
        method: "init_11",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    Row {
        group: "init",
        method: "init_12",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    Row {
        group: "init",
        method: "init_13",
        state: State::Missing,
        narrows: &[
            "forwarder only: one static-object call plus one registrar call, no behaviour to lift",
        ],
    },
    // Loudness resolvers: need the evaluator-object model.
    Row {
        group: "loudness",
        method: "loudness_resolve",
        state: State::Missing,
        narrows: &["needs the evaluator-object model plus transfer/filter callee traits"],
    },
    Row {
        group: "loudness",
        method: "loudness_resolve_scaled",
        state: State::Missing,
        narrows: &["needs the evaluator-object model plus transfer/filter callee traits"],
    },
    Row {
        group: "loudness",
        method: "loudness_resolve_gated",
        state: State::Missing,
        narrows: &["needs the evaluator-object model plus threshold/magic-rounding callee traits"],
    },
    // Entity pair: needs virtual dispatch through slot 59.
    Row {
        group: "pair",
        method: "pair_max_reach",
        state: State::Missing,
        narrows: &["needs slot-59 virtual dispatch plus the coordinate helper as a trait"],
    },
    Row {
        group: "pair",
        method: "pair_max_span",
        state: State::Missing,
        narrows: &["needs slot-59 virtual dispatch plus the coordinate helper as a trait"],
    },
    // Positional gain: needs the voice-table link model.
    Row {
        group: "positional",
        method: "positional_update",
        state: State::Missing,
        narrows: &["needs the voice-table link plus position/gain callee traits"],
    },
    // Weather entity: the next behavioural sub-group.
    Row {
        group: "weather",
        method: "update",
        state: State::Missing,
        narrows: &[
            "needs the weather-entity float block plus curve-evaluator traits; next sub-group",
        ],
    },
    Row {
        group: "weather",
        method: "tick",
        state: State::Missing,
        narrows: &[
            "needs the weather-entity float block plus curve-evaluator traits; next sub-group",
        ],
    },
    // Snapshot, reinit and bind: need their collaborator models.
    Row {
        group: "snapshot",
        method: "snapshot_copy",
        state: State::Missing,
        narrows: &["needs the wide-entity layout plus the helper callee as a trait"],
    },
    Row {
        group: "reinit",
        method: "reinit_gate",
        state: State::Missing,
        narrows: &["needs the controller/finaliser callee chain as traits"],
    },
    Row {
        group: "bind",
        method: "bind_params",
        state: State::Missing,
        narrows: &["needs the slot binder plus allocator/child-constructor traits"],
    },
];

/// Counts of (proven, lifted-but-unproven, missing) rows.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_counts_cover_all_rows() {
        let (proven, lifted, missing) = counts();
        assert_eq!((proven, lifted, missing), (5, 0, 24));
        assert_eq!(proven + lifted + missing, ROWS.len());
        assert_eq!(ROWS.len(), 29);
    }
}
