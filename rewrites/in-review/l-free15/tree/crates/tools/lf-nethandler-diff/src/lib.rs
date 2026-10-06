//! `lf-nethandler-diff`: differential tests of the lifted network handler slots.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate's runtime through the same macro and function names
//! the checker builds them against.
//!
//! The differential cases run on the 32-bit target only: the rewrites
//! take real addresses. On other hosts the crate builds but runs no
//! rewrite cases (the lifted crate's own host tests run everywhere).

pub mod rt;

pub use rt::global;
pub use rt::relocated;
pub use rt::set_relocated;

#[cfg(target_arch = "x86")]
pub mod rewrites;
