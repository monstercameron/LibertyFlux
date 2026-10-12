//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`
//! (32-bit target only). The include paths are relative to this file: from
//! this crate's directory, three levels up to the repository root, then
//! rewrites/verified/functions/.

// Byte-identical verified files; their style is not linted here.
#![allow(
    unsafe_code,
    missing_docs,
    non_snake_case,
    unused_doc_comments,
    unused_imports,
    unused_mut,
    unused_unsafe,
    unused_variables,
    clippy::all,
    clippy::pedantic
)]

pub use crate::rt::{callee_addr, global, relocated};
pub use crate::{callee_cdecl, callee_thiscall, export};

include!("../../../../../../../../rewrites/verified/functions/fn_00d6bcf0.rs");
include!("../../../../../../../../rewrites/verified/functions/fn_00d6be10.rs");
include!("../../../../../../../../rewrites/verified/functions/fn_00d70950.rs");
