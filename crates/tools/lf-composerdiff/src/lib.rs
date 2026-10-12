//! `lf-composerdiff`: differential tests of the lifted composer classes.
//!
//! The verified rewrite files are included unchanged and compiled against
//! the shared checker runtime (`lf_checker_rt`). Each lifted method runs
//! beside its rewrite on the same generated inputs. Everything here that
//! names a 32-bit calling convention or a rewrite is compiled for the
//! 32-bit target only; on other hosts the crate builds and its tests are
//! empty, while the lifted module's own host tests run everywhere.

// The rewrites and the harness read and write raw test memory; the
// lifted crate itself stays free of unsafe code.
#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
pub mod harness;

#[cfg(target_arch = "x86")]
pub mod rewrites;
