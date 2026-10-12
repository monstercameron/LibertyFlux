//! What is lifted from the layout frame, what is proven, and what each proof
//! leaves out.
//!
//! One row per verified 32-bit method of the frame's interface. `Proven`
//! means the method is restated on this module's types and the differential
//! test crate (`lf-layoutframe-diff`) ran it against its verified rewrite on
//! the same generated inputs, comparing the answer and every byte of the
//! frame (and of the shared memory the method reads or writes), with a
//! deliberately wrong lift caught alongside. `Missing` rows are verified
//! methods that are not restated.
//!
//! Nothing in this table is checker-verified. The rewrites are verified; the
//! lifted methods are proven against them, and that is what is claimed.

/// Lift state of one verified method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated and proven against its rewrite.
    Proven,
    /// Verified, but not restated; see the narrowing text for why.
    Missing,
}

/// One verified method's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// The lifted method's name, or a descriptive name for a missing method.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift leaves out, or why the method is missing. Empty means
    /// the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every proven method is modelled as the frame's span of bytes; the
/// object's size beyond the span is not modelled.
const SPAN_NARROWING: &str =
    "the frame is modelled as its span of bytes; the object's size beyond the span is not modelled";

/// A method that stores a value and has no answer: the rewrite answers
/// zero, which the proof checks, and the lifted method returns nothing.
const VOID_ANSWER: &str =
    "the rewrite answers zero; the lifted method has no answer and the proof checks that zero";

/// A copy whose rewrite answers the source address: the lifted method takes
/// the block by reference and has no answer.
const COPY_ANSWER: &str =
    "the rewrite answers the source address; the lifted method takes the block by reference and has no answer";

/// The callees of a call-carrying method write nothing into the frame or the
/// shared bytes in the proof; a callee that wrote them would need its writes
/// scripted.
const CALLEES_WRITE_NOTHING: &str =
    "the callees write nothing into the frame or the shared bytes in the proof; a callee that did would need its writes scripted";

/// Shared bytes are compared by their final values, not by the order of the
/// reads and writes of them.
const SHARED_FINAL_VALUES: &str =
    "shared bytes are compared by their final values, not by the order of their reads and writes";

/// Every verified method of the frame's interface, in the order the lift
/// reached them.
pub const ROWS: &[Row] = &[
    // Call-free: field stores, byte masks, block copies, and two branches.
    Row { method: "set_flag_c4", state: State::Proven, narrows: &[SPAN_NARROWING] },
    Row { method: "set_flag_c7", state: State::Proven, narrows: &[SPAN_NARROWING] },
    Row { method: "set_flag_c8", state: State::Proven, narrows: &[SPAN_NARROWING] },
    Row { method: "set_flag_ca", state: State::Proven, narrows: &[SPAN_NARROWING] },
    Row { method: "set_flag_d0", state: State::Proven, narrows: &[SPAN_NARROWING] },
    Row { method: "set_flag_d1", state: State::Proven, narrows: &[SPAN_NARROWING] },
    Row { method: "set_flag_eb", state: State::Proven, narrows: &[SPAN_NARROWING] },
    Row { method: "set_flag_c9", state: State::Proven, narrows: &[SPAN_NARROWING, VOID_ANSWER] },
    Row { method: "set_flag_e8", state: State::Proven, narrows: &[SPAN_NARROWING, VOID_ANSWER] },
    Row { method: "set_flag_e9", state: State::Proven, narrows: &[SPAN_NARROWING, VOID_ANSWER] },
    Row { method: "set_flag_ea", state: State::Proven, narrows: &[SPAN_NARROWING, VOID_ANSWER] },
    Row { method: "set_word_a0", state: State::Proven, narrows: &[SPAN_NARROWING] },
    Row { method: "set_word_d8", state: State::Proven, narrows: &[SPAN_NARROWING] },
    Row { method: "set_word_e0", state: State::Proven, narrows: &[SPAN_NARROWING] },
    Row { method: "set_word_e4", state: State::Proven, narrows: &[SPAN_NARROWING] },
    Row { method: "set_word_d4", state: State::Proven, narrows: &[SPAN_NARROWING, VOID_ANSWER] },
    Row { method: "set_word_dc", state: State::Proven, narrows: &[SPAN_NARROWING, VOID_ANSWER] },
    Row { method: "set_float_4", state: State::Proven, narrows: &[SPAN_NARROWING, VOID_ANSWER] },
    Row { method: "set_float_8", state: State::Proven, narrows: &[SPAN_NARROWING, VOID_ANSWER] },
    Row {
        method: "store_float_b0_clear_tags",
        state: State::Proven,
        narrows: &[SPAN_NARROWING, VOID_ANSWER],
    },
    Row { method: "store_float_b0_with_tag", state: State::Proven, narrows: &[SPAN_NARROWING] },
    Row {
        method: "set_slot_and_fraction",
        state: State::Proven,
        narrows: &[SPAN_NARROWING, VOID_ANSWER],
    },
    Row {
        method: "install_cache_if_empty",
        state: State::Proven,
        narrows: &[SPAN_NARROWING, VOID_ANSWER],
    },
    Row { method: "copy_block_to_10", state: State::Proven, narrows: &[SPAN_NARROWING, COPY_ANSWER] },
    Row { method: "copy_block_to_a4", state: State::Proven, narrows: &[SPAN_NARROWING, COPY_ANSWER] },
    // Calls out: the token family, the blend, the record query, the list
    // release, the indexed check and the parameter-block refresh.
    Row {
        method: "forward_one_engine_call",
        state: State::Missing,
        narrows: &["one forwarded engine call with two frame fields and a constant; no behaviour of its own, so it is not lifted"],
    },
    Row {
        method: "resolve_token_and_follow_up_plus_one",
        state: State::Proven,
        narrows: &[SPAN_NARROWING, CALLEES_WRITE_NOTHING],
    },
    Row {
        method: "token_follow_up_then_two_enables",
        state: State::Proven,
        narrows: &[SPAN_NARROWING, CALLEES_WRITE_NOTHING],
    },
    Row {
        method: "token_follow_up_then_reset",
        state: State::Proven,
        narrows: &[SPAN_NARROWING, CALLEES_WRITE_NOTHING],
    },
    Row {
        method: "token_follow_up_when_nonzero",
        state: State::Proven,
        narrows: &[SPAN_NARROWING, CALLEES_WRITE_NOTHING],
    },
    Row {
        method: "token_follow_up_then_clear_pending",
        state: State::Proven,
        narrows: &[SPAN_NARROWING, CALLEES_WRITE_NOTHING, SHARED_FINAL_VALUES],
    },
    Row {
        method: "three_gated_follow_ups_then_clear_pending",
        state: State::Proven,
        narrows: &[SPAN_NARROWING, CALLEES_WRITE_NOTHING, SHARED_FINAL_VALUES],
    },
    Row {
        method: "token_follow_up_then_handoff",
        state: State::Proven,
        narrows: &[SPAN_NARROWING, CALLEES_WRITE_NOTHING, SHARED_FINAL_VALUES],
    },
    Row {
        method: "store_mode_then_blend_and_forward",
        state: State::Proven,
        narrows: &[
            SPAN_NARROWING,
            "the two shared constants are supplied by the host; the blend handler's answer is discarded and the method answers zero",
        ],
    },
    Row {
        method: "two_flags_set_among_records",
        state: State::Proven,
        narrows: &[
            SPAN_NARROWING,
            "the host answers the container and the record flags; the record pointers are read by the host",
        ],
    },
    Row {
        method: "release_pointer_list_entries",
        state: State::Proven,
        narrows: &[
            SPAN_NARROWING,
            CALLEES_WRITE_NOTHING,
            "the list bounds are read once before the walk; the owner receives the frame, and the field address is compared as an offset",
        ],
    },
    Row {
        method: "every_list_entry_resolves",
        state: State::Proven,
        narrows: &[
            SPAN_NARROWING,
            CALLEES_WRITE_NOTHING,
            "the fallback answer is the frame's own list field, answered as an enum; the first query is compared by its low byte",
        ],
    },
    Row {
        method: "refresh_parameter_block",
        state: State::Proven,
        narrows: &[
            SPAN_NARROWING,
            CALLEES_WRITE_NOTHING,
            SHARED_FINAL_VALUES,
            "the allocator never answers zero in the proof: the original's allocation-failure path writes through the null block and faults",
        ],
    },
];

/// Counts of the rows in each state.
#[must_use]
pub fn count(state: State) -> usize {
    ROWS.iter().filter(|row| row.state == state).count()
}
