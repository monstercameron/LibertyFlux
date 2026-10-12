//! `lf-peds-diff`: differential tests of the lifted move-between-points scenario
//! methods against their verified 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target they see
//! this crate as `lf_checker_rt` via `extern crate self as`, with the same macro
//! and function names the checker builds them against. The rewrites take real
//! addresses, so the differential cases run on the 32-bit target only; on other
//! hosts the crate builds and runs no rewrite case.

// Test-only runtime: the rewrites take raw addresses, and the harness hands them
// the addresses of its own buffers and stubs. The lifted crate itself has no unsafe.
#![allow(unsafe_code)]

// The crate root re-exports the runtime surface under the checker's name so that
// `lf_checker_rt::...` paths in the included verified files resolve here.
extern crate self as lf_checker_rt;

pub mod rt;

pub use rt::{callee_addr, relocated};

#[cfg(target_arch = "x86")]
pub mod rewrites;
