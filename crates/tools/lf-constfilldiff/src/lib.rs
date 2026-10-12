//! `lf-constfilldiff`: differential tests of the lifted constant fill
//! (`WordFill::apply` in `lf-core`, module `const_fill`) against its verified
//! 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target they
//! see this crate as `lf_checker_rt` (and one of them as `lf_k2_rt`) via
//! `extern crate self as`. The differential cases run on the 32-bit target
//! only. On other hosts the crate builds and the lifted method's host tests run
//! in `lf-core`.

// The crate root re-exports the runtime surface under the checker's names so
// the `lf_checker_rt::` and `lf_k2_rt::` paths in the included files resolve here.
extern crate self as lf_checker_rt;
extern crate self as lf_k2_rt;

#[cfg(target_arch = "x86")]
#[macro_use]
pub mod rt;

#[cfg(target_arch = "x86")]
pub use rt::global;

#[cfg(target_arch = "x86")]
pub mod rewrites;
