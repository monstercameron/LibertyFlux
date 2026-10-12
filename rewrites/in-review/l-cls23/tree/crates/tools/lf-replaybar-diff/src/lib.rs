//! `lf-replaybar-diff`: lifted replay progress bar methods against their
//! verified 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` via `extern crate self as`, with
//! the same macro and function names the checker builds them against.
//! Each numbered callee of a rewrite is a recording stub (`rt`); the
//! lifted side runs against a recorder (`fake`) with the same scripted
//! answers. Both logs are compared as text: every collaborator call in
//! order with its arguments, every returned word, the bytes of the object
//! and the globals.
//!
//! The differential cases run on the 32-bit target only: the rewrites take
//! real addresses, so the runtime and the rewrites are compiled only there.
//! The recorder and the wrong lifts build on every host.

// The rewrites use the runtime surface under the checker's names.
#[cfg(target_arch = "x86")]
extern crate self as lf_checker_rt;

pub mod fake;
pub mod mutants;

#[cfg(target_arch = "x86")]
pub mod rt;
#[cfg(target_arch = "x86")]
pub mod rewrites;

#[cfg(target_arch = "x86")]
pub use rt::{callee_addr, global, relocated};
