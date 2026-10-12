//! `lf-layoutframe-diff`: differential tests of the lifted layout-frame methods.
//!
//! The verified rewrite files are included unchanged. On the 32-bit target
//! they see this crate's `lf_checker_rt` dependency, the shared checker
//! runtime, through the same macro names they are built against. The
//! differential cases in `tests/diff.rs` run each lifted method and its
//! rewrite on the same generated inputs and compare the answer and every
//! byte of the frame, and a deliberately wrong lift of each method must be
//! caught.
//!
//! The rewrites take real object addresses, so they exist on the 32-bit
//! target only. On other hosts the crate builds with nothing in it; the
//! lifted crate's own host tests run everywhere.

#[cfg(target_arch = "x86")]
pub mod rewrites;
