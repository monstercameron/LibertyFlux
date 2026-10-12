//! `lf-24-packdiff`: differential tests of the existing lifted packed_flag routine against its verified 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target they see this crate as
//! `lf_checker_rt` via `extern crate self as`. The differential tests run on the 32-bit target only,
//! because the rewrites take real addresses. On other hosts the crate builds and runs the host tests of
//! the instance table.

// The crate root re-exports the runtime surface under the checker's names so `lf_checker_rt::...`
// paths in the included verified files resolve here.
extern crate self as lf_checker_rt;

#[cfg(target_arch = "x86")]
pub mod rt;

#[cfg(target_arch = "x86")]
pub mod rewrites;

#[cfg(target_arch = "x86")]
pub mod calls;

pub mod instances;
