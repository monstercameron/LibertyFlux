//! `lf-formats`: shared support for game file-format readers.
//!
//! README for future lanes:
//! - Per-format reader crates from the fmt-* lanes (rpf, img, rsc5, ...)
//!   live beside this crate in `crates/formats/` as siblings, not inside
//!   it. Each gets its own crate so licences and dependencies stay
//!   separable (some formats need flagged decoders; see t-engine-crates).
//! - This crate holds only what readers share: error types, binary-reading
//!   helpers and re-exports (expected: `binrw`, `nom`, `byteorder`).
//! - Format readers are portable tools code. They never contain original
//!   game code; parsed structures describe file bytes, not game memory.

/// Placeholder error type; real variants arrive with the first reader.
#[derive(Debug)]
pub enum FormatError {
    /// The input ended before the value was complete.
    UnexpectedEnd,
    /// The input claims a shape this reader does not support.
    Unsupported(&'static str),
}

impl core::fmt::Display for FormatError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnexpectedEnd => f.write_str("unexpected end of input"),
            Self::Unsupported(what) => write!(f, "unsupported: {what}"),
        }
    }
}

impl std::error::Error for FormatError {}
