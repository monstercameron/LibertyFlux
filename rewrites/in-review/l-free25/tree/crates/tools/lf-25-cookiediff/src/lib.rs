//! `lf-25-cookiediff`: differential tests of the lifted cookie-seal routine
//! (`CookieSeal::seal` in `lf-core`) against its verified 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target they
//! see this crate as `lf_checker_rt` via `extern crate self as`. The
//! differential cases run on the 32-bit target only.

// The crate root re-exports the runtime surface under the checker's names so
// `lf_checker_rt::...` paths in the included verified files resolve here.
extern crate self as lf_checker_rt;

#[cfg(target_arch = "x86")]
pub mod rt;

#[cfg(target_arch = "x86")]
pub use rt::{global, relocated};

#[cfg(target_arch = "x86")]
pub mod rewrites;
