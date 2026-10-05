//! `lf-fam-diff`: differential tests of the lifted families.
//!
//! Lane l-lb2 draft, modelled on l-lb1's `lf-lb-diff`: the verified
//! rewrite files are included unchanged; on the 32-bit target they see
//! this crate as their checker runtime (via `extern crate self as`), with
//! the same macro and function names the checker builds them against.
//! Lane runtime prefixes (`lf_k2_rt`, ...) resolve to the same surface.
//!
//! The differential cases run on the 32-bit target only (the rewrites call
//! through real addresses). On other hosts the crate builds but runs no
//! rewrite cases; the lifted crates' own tests run everywhere.

// The crate root re-exports the runtime surface under every name the
// included files use, so their `use` and qualified paths resolve here.
extern crate self as lf_checker_rt;
extern crate self as lf_k2_rt;
extern crate self as lf_rn94_rt;
extern crate self as lf_rn35_rt;
extern crate self as lf_rn54_rt;
extern crate self as lf_rn109_rt;
extern crate self as lf_rn101_rt;
extern crate self as lf_rn90_rt;

pub mod rt;

pub use rt::callee_addr;
pub use rt::global;
pub use rt::relocated;

#[cfg(target_arch = "x86")]
pub mod rewrites;
