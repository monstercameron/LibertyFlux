//! `lf-lb-diff`: differential tests of the lifted leaderboard slots.
//!
//! Lane l-lb1 draft, modelled on `lf-lift-diff` with one addition the
//! family needs: the fetch callee fills an out-block, so the 32-bit stubs
//! write the scripted count and table pointers through the block address
//! the rewrite passes. That only works where the address is real, so the
//! differential cases run on the 32-bit target only; on other hosts the
//! crate builds but runs no rewrite cases (the lifted crate's own tests
//! run everywhere).
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` (via `extern crate self as`),
//! with the same macro and function names the checker builds them against.

// The crate root re-exports the runtime surface under the checker's name so
// `use lf_checker_rt::{...}` and `lf_checker_rt::...` paths in the included
// verified files resolve here.
extern crate self as lf_checker_rt;

pub mod rt;

pub use rt::callee_addr;
pub use rt::relocated;

#[cfg(target_arch = "x86")]
pub mod rewrites;
