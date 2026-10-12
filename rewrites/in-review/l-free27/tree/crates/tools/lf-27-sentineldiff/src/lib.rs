//! `lf-27-sentineldiff`: differential tests of the lifted sentinel-table reset
//! (`SentinelTable::fill` in `lf-core`, module `sentinel_table`) against four
//! verified 32-bit table resets of the same shape.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` via `extern crate self as`, with the
//! same macro and function names the checker builds them against. The
//! runtime maps one window of file addresses onto test memory (see `rt`).
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
