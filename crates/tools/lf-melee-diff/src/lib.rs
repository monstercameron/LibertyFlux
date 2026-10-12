//! `lf-melee-diff`: differential tests of the lifted complex melee task
//! methods against their verified 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged. They use the shared
//! checker runtime (`lf-checker-rt`) for callees, globals and the export
//! macro; the differential cases point that runtime at test memory. The
//! rewrites exist only on the 32-bit target, so the module is compiled there
//! alone; on other hosts this crate builds and runs no rewrite cases.

#[cfg(target_arch = "x86")]
pub mod rewrites;
