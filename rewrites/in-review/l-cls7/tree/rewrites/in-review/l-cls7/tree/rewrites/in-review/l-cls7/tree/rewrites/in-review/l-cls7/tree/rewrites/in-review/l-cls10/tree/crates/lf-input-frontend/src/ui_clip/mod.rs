//! Lifted UI clips: screen elements that play a short sequence.
//!
//! The original shows transient interface over the game through clip
//! objects ([`BasicClip`]): a flag byte, a mode byte, a stored float and
//! four collaborator objects, with a text sink and a replay progress bar
//! drawn alongside. This module lifts the clip's verified behaviour to
//! ordinary Rust: the clip owns its words and its element array, the
//! collaborators travel as opaque cookies through [`ClipWorld`], and
//! each verified 32-bit method with behaviour in it is restated as a
//! method. The replay progress bar's four verified methods are recorded
//! in [`registry`] but not lifted yet: each needs a model this module
//! does not build (see the rows' reasons).
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results, every
//! written byte and every collaborator call in order, floats bit for
//! bit (see the `lf-uiclip-diff` test crate). Nothing here is verified
//! by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].

#![forbid(unsafe_code)]

mod clip;

pub mod registry;

pub use clip::{
    BasicClip, ClipWorld, ElementTag, EntryTag, EntryTableTag, MEASURE_BIAS, MEASURE_SCALE,
    MatchOut, PartTag, SinkTag, SubmitTag, TRIPLE_C0, TRIPLE_C1, TRIPLE_C2, TransformRecord,
    TripleKind, quiet_snan, triple_bytes, truncate_raw, up_to_nul,
};
