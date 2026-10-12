//! `lf-taskflagdiff`: differential tests of the lifted task-id flag-bit read
//! (`TaskFlagBit::bit_of_entry` in `lf-core`, module `task_flags`) against
//! its verified 32-bit rewrites.
//!
//! The verified rewrite files are read unchanged; on the 32-bit target they see
//! this crate as `lf_checker_rt` via `extern crate self as`. The differential
//! cases run on the 32-bit target only.

extern crate self as lf_checker_rt;

#[cfg(target_arch = "x86")]
#[macro_use]
pub mod rt;

#[cfg(target_arch = "x86")]
pub use rt::relocated;

#[cfg(target_arch = "x86")]
pub mod rewrites;

