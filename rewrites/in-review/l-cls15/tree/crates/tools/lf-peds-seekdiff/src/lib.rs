//! `lf-peds-seekdiff`: differential tests of the lifted seek-cover tasks
//! against their verified 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged. On the 32-bit target
//! they see this crate as `lf_checker_rt` (via `extern crate self as`),
//! with the macro and function names the checker builds them against. The
//! differential cases run on the 32-bit target only, because the rewrites
//! take real addresses. On other hosts the crate builds and runs no
//! rewrite case; the lifted crate's own host tests run everywhere.

// The crate root re-exports the runtime surface under the checker's names
// so `use lf_checker_rt::{...}` and `lf_checker_rt::...` paths in the
// included verified files resolve here.
extern crate self as lf_checker_rt;

pub mod rt;

pub use rt::callee_addr;
pub use rt::global;
pub use rt::relocated;

#[cfg(target_arch = "x86")]
pub mod rewrites;
