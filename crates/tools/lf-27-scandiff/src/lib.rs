//! `lf-27-scandiff`: differential tests of the lifted counted-run search
//! (`CountedRun::evaluate` in `lf-core`) against eight verified 32-bit
//! rewrites of the same family.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` via `extern crate self as`, with the
//! same macro and function names the checker builds them against. The
//! rewrites in this family read and write only the object they are given, so
//! the runtime needs no globals and no relocation.
//!
//! The differential cases run on the 32-bit target only, because the rewrites
//! take real addresses. On other hosts the crate builds and runs the host
//! edge tests of the lifted method.

// The crate root re-exports the runtime surface under the checker's names
// so `lf_checker_rt::...` paths in the included verified files resolve here.
extern crate self as lf_checker_rt;

pub mod rt;

pub use rt::{global, relocated};

#[cfg(target_arch = "x86")]
pub mod rewrites;
