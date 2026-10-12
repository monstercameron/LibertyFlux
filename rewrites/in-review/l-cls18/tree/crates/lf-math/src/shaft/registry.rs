//! What is lifted and proven, and what each proof narrows.
//!
//! Each row names a step method of [`super::Shaft`], the channel it
//! advances, the differential test that proves it, and what that proof does
//! not cover. "Proven" means: agreement with the verified 32-bit rewrite of
//! the same step on the test's generated inputs, comparing the returned
//! value, every word the step writes, the pending flag and every radicand
//! the root helper is asked for, floats bit for bit. Nothing here is
//! verified by the checker itself.

/// One lifted step method and its proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Proof {
    /// The method on [`super::Shaft`].
    pub method: &'static str,
    /// The channel it advances (0 to 3).
    pub channel: usize,
    /// The differential test that proves it.
    pub proof: &'static str,
    /// What the proof does not cover.
    pub narrows: &'static str,
}

/// Narrowing shared by every step.
const SHARED: &str = "The root helper is a callee: both sides call the host square root \
    and record the radicand; the helper's own effects are not modelled. The \
    constants (two, one half, one, minus one) are the values the executable holds. \
    Inputs are generated, not exhaustive. The pending flag's start value is not read. \
    The class constructor and the matrix update are not lifted.";

/// The lifted step methods, one row per channel.
pub const PROOFS: [Proof; 4] = [
    Proof {
        method: "advance_0",
        channel: 0,
        proof: "lf-shaftdiff tests/diff.rs: channel 0 against its verified rewrite",
        narrows: SHARED,
    },
    Proof {
        method: "advance_1",
        channel: 1,
        proof: "lf-shaftdiff tests/diff.rs: channel 1 against its verified rewrite",
        narrows: SHARED,
    },
    Proof {
        method: "advance_2",
        channel: 2,
        proof: "lf-shaftdiff tests/diff.rs: channel 2 against its verified rewrite",
        narrows: SHARED,
    },
    Proof {
        method: "advance_3",
        channel: 3,
        proof: "lf-shaftdiff tests/diff.rs: channel 3 against its verified rewrite",
        narrows: SHARED,
    },
];
