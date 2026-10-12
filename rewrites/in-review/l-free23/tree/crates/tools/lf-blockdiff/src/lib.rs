//! `lf-blockdiff`: differential tests of the lifted three- and four-word block
//! copy (`WordBlockCopy::run` in `lf-core`, module `word_block`) against its
//! verified 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` via `extern crate self as`, and they
//! use the runtime's `export!` macro under that name. The differential cases
//! run on the 32-bit target only. On other hosts the crate builds and the
//! lifted method's host tests run in `lf-core`.

// The crate root re-exports the runtime surface under the checker's names
// so `lf_checker_rt::...` paths in the included verified files resolve here.
extern crate self as lf_checker_rt;

#[cfg(target_arch = "x86")]
#[macro_use]
pub mod rt;

#[cfg(target_arch = "x86")]
pub mod rewrites;
