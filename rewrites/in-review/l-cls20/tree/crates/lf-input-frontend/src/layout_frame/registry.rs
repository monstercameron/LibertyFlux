//! What is lifted from the layout frame, what is proven, and what each proof
//! leaves out.
//!
//! One row per verified 32-bit method of the frame's interface. `Proven`
//! means the method is restated on [`UILayoutFrame`](super::frame::UILayoutFrame)
//! and the differential test crate (`lf-layoutframe-diff`) ran it against
//! its verified rewrite on the same generated inputs, comparing the answer
//! and every byte of the frame, with a deliberately wrong lift caught
//! alongside. `Missing` rows call other code or read shared state; they wait
//! for the trait that carries their callees and are not restated here.
//!
//! Nothing in this table is checker-verified. The rewrites are verified;
//! the lifted methods are proven against them, and that is what is claimed.

/// Lift state of one verified method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on the frame type and proven against its rewrite.
    Proven,
    /// Verified, but it calls other code or reads shared state. Not
    /// restated yet.
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

/// Every verified method of the frame's interface, in the order the lift
/// reached them.
pub const ROWS: &[Row] = &[
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
    Row {
        method: "forward_one_engine_call",
        state: State::Missing,
        narrows: &["one forwarded engine call with two frame fields and a constant; no behaviour of its own, so it is not lifted"],
    },
    Row {
        method: "resolve_token_and_follow_up_plus_one",
        state: State::Missing,
        narrows: &["calls the shared token resolver, then a virtual token query and a virtual follow-up on the resolved target"],
    },
    Row {
        method: "store_mode_then_blend_and_forward",
        state: State::Missing,
        narrows: &["reads two shared float constants, calls a virtual blend handler; the blend arithmetic is single precision"],
    },
    Row {
        method: "list_has_both_record_flags",
        state: State::Missing,
        narrows: &["calls a virtual query for a counted record list; the list records are read directly"],
    },
    Row {
        method: "every_list_entry_resolves",
        state: State::Missing,
        narrows: &["calls a virtual query, a static registry lookup and an indexed query per list entry"],
    },
    Row {
        method: "release_list_entries",
        state: State::Missing,
        narrows: &["calls each live entry's own teardown and free routine, then the list owner's callback"],
    },
    Row {
        method: "token_follow_up_then_two_enables",
        state: State::Missing,
        narrows: &["calls the token resolver and virtual steps, then two virtual enable steps on the frame"],
    },
    Row {
        method: "token_follow_up_then_reset",
        state: State::Missing,
        narrows: &["calls the token resolver and virtual steps, then a virtual reset step on the frame"],
    },
    Row {
        method: "token_follow_up_when_nonzero",
        state: State::Missing,
        narrows: &["calls the token resolver and virtual steps; a zero token answers zero"],
    },
    Row {
        method: "three_gated_follow_ups_then_clear_pending",
        state: State::Missing,
        narrows: &["calls the token resolver, virtual steps, a shared sink object and the shared notifier, reads and clears a global flag"],
    },
    Row {
        method: "token_follow_up_then_clear_pending",
        state: State::Missing,
        narrows: &["calls the token resolver, virtual steps and the shared notifier, reads and clears a global flag"],
    },
    Row {
        method: "token_follow_up_then_handoff",
        state: State::Missing,
        narrows: &["calls the token resolver, virtual steps and a shared handoff routine through a global object"],
    },
    Row {
        method: "refresh_parameter_block",
        state: State::Missing,
        narrows: &["calls the allocator for the block, a virtual stamp and reads and writes the block's words"],
    },
];

/// Counts of the rows in each state.
#[must_use]
pub fn count(state: State) -> usize {
    ROWS.iter().filter(|row| row.state == state).count()
}
