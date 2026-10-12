//! `lf-mirrordiff`: the lifted mirror-reflection render phase against its
//! verified 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` via `extern crate self as`, with the
//! same macro and function names the checker builds them against. Each
//! differential case builds the phase's object, parameter block, snapshot and
//! globals in test memory, runs the rewrite and the lift on identical copies,
//! and compares the results, every recorded call in order with its arguments
//! and the words its pointer arguments held, and the whole memory image.
//!
//! The differential cases run on the 32-bit target only: the rewrites take
//! real addresses. On other hosts the crate builds and runs no case.

// The crate root re-exports the runtime surface under the checker's names
// so `use lf_checker_rt::{...}` and `lf_checker_rt::...` paths in the
// included verified files resolve here.
extern crate self as lf_checker_rt;

#[cfg(target_arch = "x86")]
pub mod rt;

#[cfg(target_arch = "x86")]
pub use rt::callee_addr;
#[cfg(target_arch = "x86")]
pub use rt::global;
#[cfg(target_arch = "x86")]
pub use rt::relocated;

#[cfg(target_arch = "x86")]
pub mod rewrites;

#[cfg(target_arch = "x86")]
pub mod harness;

#[cfg(target_arch = "x86")]
pub mod mutants;

