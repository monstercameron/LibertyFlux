//! `lf-animchan-diff`: differential tests of the lifted animation channels.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` (and `lf_k2_rt` for the one
//! file written against that dialect) via `extern crate self as`, with
//! the same macro and function names the checker builds them against.
//! The shared float constants the samplers read come from a table of the
//! values measured from the executable's data.
//!
//! The differential cases run on the 32-bit target only: the rewrites
//! take real addresses. On other hosts the crate builds but runs no
//! rewrite cases (the lifted crate's own host tests run everywhere).

// The crate root re-exports the runtime surface under the checker's names
// so `use lf_checker_rt::{...}` and `lf_checker_rt::...` paths in the
// included verified files resolve here.
extern crate self as lf_checker_rt;
extern crate self as lf_k2_rt;

pub mod rt;

pub use rt::callee_addr;
pub use rt::global;
pub use rt::relocated;

#[cfg(target_arch = "x86")]
pub mod rewrites;
