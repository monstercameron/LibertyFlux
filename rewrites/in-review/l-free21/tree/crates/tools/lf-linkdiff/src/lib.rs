//! `lf-linkdiff`: differential tests of the lifted head-swap (`StaticLinks::replace_head`
//! in `lf-core`) against a batch of its verified 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` via `extern crate self as`, with the
//! same macro and function names the checker builds them against. This crate
//! holds batch 1 of the instances; the others are in the sibling crates.
//! The differential cases run on the 32-bit target only, because the rewrites
//! take real addresses. On other hosts the crate builds and runs only the
//! host edge tests of the lifted method.

// The crate root re-exports the runtime surface under the checker's names
// so `lf_checker_rt::...` paths in the included verified files resolve here.
extern crate self as lf_checker_rt;

#[cfg(target_arch = "x86")]
pub mod rt;

#[cfg(target_arch = "x86")]
pub use rt::{global, plant, read_back, relocated};

#[cfg(target_arch = "x86")]
pub mod rewrites;
