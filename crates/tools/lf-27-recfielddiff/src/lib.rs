//! `lf-27-recfielddiff`: differential tests of the lifted record field access
//! (`RecordStore::store` and `RecordFlag::read` in `lf-core`, module
//! `record_field_access`) against four verified 32-bit routines that store into,
//! or read a flag of, the record a stream index selects.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target they
//! see this crate as `lf_checker_rt` via `extern crate self as`. The runtime maps
//! one window of file addresses onto test memory (see `rt`): the record and
//! validity tables are inside it, and the record blocks they point to are
//! ordinary test memory.
//!
//! The differential cases run on the 32-bit target only, because the rewrites
//! take real addresses. On other hosts the crate builds and runs no cases.

// The crate root re-exports the runtime surface under the checker's names
// so `lf_checker_rt::...` paths in the included verified files resolve here.
extern crate self as lf_checker_rt;

pub mod rt;

pub use rt::{global, relocated};

#[cfg(target_arch = "x86")]
pub mod rewrites;
