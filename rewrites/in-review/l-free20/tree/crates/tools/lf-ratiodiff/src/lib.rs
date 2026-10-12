//! `lf-ratiodiff`: differential tests of the lifted ratio routine against a batch
//! of its verified 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` via `extern crate self as`, with the
//! same macro and function names the checker builds them against. This crate
//! holds batch 1 of the instances; the others are in the sibling crates.
//! The differential cases run on the 32-bit target only, because the rewrites
//! take real addresses. On other hosts the crate builds and runs no rewrite.

// The crate root re-exports the runtime surface under the checker's names
// so `lf_checker_rt::...` paths in the included verified files resolve here.
extern crate self as lf_checker_rt;

pub mod rt;

pub use rt::global;
pub use rt::plant;
pub use rt::read_back;

#[cfg(target_arch = "x86")]
pub mod rewrites;
