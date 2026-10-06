//! `lf-taskdiff`: differential tests of the lifted pedestrian tasks.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` via `extern crate self as`, with
//! the same macro and function names the checker builds them against.
//! Rewrites using the bare `export!` form import the macros from the crate
//! root in their module (see `rewrites_gen.rs`). Each diff test binary
//! registers its own callee stubs and scripts the globals its method reads.
//!
//! The differential cases run on the 32-bit target only: the rewrites
//! take real addresses. On other hosts the crate builds but runs no
//! rewrite cases (the lifted crate's own host tests run everywhere).

// The crate root re-exports the runtime surface under the checker's name
// so qualified paths in the included verified files resolve here.
extern crate self as lf_checker_rt;

pub mod rt;

pub use rt::callee_addr;
pub use rt::global;
pub use rt::relocated;
pub use rt::set_callee;
pub use rt::set_manager;
pub use rt::set_one;
pub use rt::set_tick;

#[cfg(target_arch = "x86")]
pub mod rewrites;
