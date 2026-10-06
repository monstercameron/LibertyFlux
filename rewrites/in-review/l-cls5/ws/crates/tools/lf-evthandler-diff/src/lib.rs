//! `lf-evthandler-diff`: differential tests of the lifted event handler.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` or `lf_rs75_rt` (one per
//! verifying lane) via `extern crate self as`, with the same macro and
//! function names the checker builds them against. The call-free cases
//! plant fake virtual tables pointing at recording stubs; the
//! factory-pair and refresh cases register callee stubs for slots 1 and
//! 2 and script the shared manager word.
//!
//! The differential cases run on the 32-bit target only: the rewrites
//! take real addresses. On other hosts the crate builds but runs no
//! rewrite cases (the lifted crate's own host tests run everywhere).

// The crate root re-exports the runtime surface under the checkers' names
// so qualified paths in the included verified files resolve here.
extern crate self as lf_checker_rt;
extern crate self as lf_rs75_rt;

pub mod rt;

pub use rt::callee_addr;
pub use rt::global;
pub use rt::relocated;
pub use rt::set_callee1;
pub use rt::set_callee2;
pub use rt::set_manager;

#[cfg(target_arch = "x86")]
pub mod rewrites;
