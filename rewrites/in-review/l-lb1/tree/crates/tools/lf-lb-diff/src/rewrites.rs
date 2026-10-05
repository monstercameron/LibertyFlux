//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`
//! (32-bit target only). The include path below is relative to this
//! manifest: `../../../../rewrites/...` from the lane's `tree/` copy,
//! `../../../rewrites/...` once integrated into the repository.

// Byte-identical verified files; their style is not linted here.
#![allow(
    unsafe_code,
    missing_docs,
    non_snake_case,
    unused_doc_comments,
    unused_imports,
    unused_mut,
    unused_variables,
    clippy::all,
    clippy::pedantic
)]

use crate::callee_addr;
use crate::callee_fastcall;
use crate::callee_thiscall;
use crate::export;
use crate::relocated;

/// One lane's shared table-read helper, as its rewrites call it: word `i`
/// of the table at `base`.
///
/// Several verified files call bare `rd(base, i)` with no import: in the
/// lane crate that built them the helper sat at the crate root, and paths
/// resolve up the module tree. It sits here for the same reason. Files that
/// define their own top-level `rd` keep theirs (the inner item shadows this
/// one), so behaviour is whatever each file was verified with.
///
/// # Caller contract
///
/// `base` points at readable words (in the differential tests, into the
/// test image) and `i` is in range.
#[allow(clippy::missing_panics_doc)]
pub fn rd(base: u32, i: u32) -> u32 {
    unsafe { (base.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned() }
}

include!("rewrites_gen.rs");
