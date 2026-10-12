//! `lf-peds-follow-diff`: differential tests of the lifted follow-point route
//! task against its verified 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged, as modules of the
//! `rewrites` module. They are compiled against the shared checker runtime
//! (`lf-checker-rt`), whose callee table this crate fills with its own
//! stubs before each call. The rewrites take real addresses, so the
//! differential cases run on the 32-bit target only; on other hosts the
//! crate builds and runs no rewrite case (the lifted crate's own host
//! tests run everywhere).

#[cfg(target_arch = "x86")]
pub mod rewrites;
