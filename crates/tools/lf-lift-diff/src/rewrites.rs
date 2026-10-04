//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file in `rewrites/verified/functions/` is included unchanged. On
//! the 32-bit target the files see the real checker runtime, exactly as the
//! checker builds them. On other hosts they see the stand-in below: the
//! same macro and function names, with `export!` producing a plain Rust
//! function and the callee macros recording through [`crate::rt::record`].
//!
//! The stand-in changes how a rewrite is called, never what its body
//! computes: the body is the tracked text. What the stand-in cannot do is
//! give a rewrite a 32-bit address it may dereference itself; the one such
//! rewrite here (`rw_00e63c90`) is exercised only on the 32-bit target.

// The rewrites are checker-form code: they dereference raw addresses into
// the test image inside their own `unsafe` blocks, by design. They are
// kept byte-identical to the verified files, so their style is not
// linted here, and their doc comments sit on macro invocations.
#![allow(
    unsafe_code,
    missing_docs,
    non_snake_case,
    unused_doc_comments,
    unused_imports,
    clippy::all,
    clippy::pedantic
)]

#[cfg(target_arch = "x86")]
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global, relocated};

// Some rewrites name the runtime by the crate their lane built against;
// those were all copies of the checker runtime.
#[cfg(target_arch = "x86")]
use lf_checker_rt as lf_rs75_rt;
#[cfg(target_arch = "x86")]
use lf_checker_rt as lf_rs89_rt;

/// The host stand-in for the checker runtime's macros.
#[cfg(not(target_arch = "x86"))]
mod stand_in {
    /// `export!` without a calling convention: a plain function.
    macro_rules! export {
        ($conv:ident, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
            #[doc = concat!("Verified rewrite `", stringify!($name), "` (host stand-in runtime).")]
            pub fn $name($($arg : $ty),*) -> $ret $body
        };
    }

    /// Every callee macro: record the call, return its scripted answer.
    macro_rules! callee {
        ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
            let args: &[u32] = &[$($arg),*];
            $crate::rt::record($id, args) as $ret
        }};
    }

    pub(crate) use callee as callee_cdecl;
    pub(crate) use callee as callee_stdcall;
    pub(crate) use callee as callee_thiscall;
    pub(crate) use export;
}

#[cfg(not(target_arch = "x86"))]
use crate::rt::{global, relocated};
#[cfg(not(target_arch = "x86"))]
use stand_in::{callee_cdecl, callee_stdcall, callee_thiscall, export};

/// The stand-in under the runtime's crate name, for rewrites that call it
/// by path.
#[cfg(not(target_arch = "x86"))]
mod lf_checker_rt {
    pub(crate) use super::stand_in::{callee_cdecl, callee_stdcall, callee_thiscall, export};
}

#[cfg(not(target_arch = "x86"))]
use self::lf_checker_rt as lf_rs75_rt;
#[cfg(not(target_arch = "x86"))]
use self::lf_checker_rt as lf_rs89_rt;

/// Includes verified rewrite files by their base name.
macro_rules! verified {
    ($($file:literal),* $(,)?) => {
        $(include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../rewrites/verified/functions/",
            $file,
            ".rs"
        ));)*
    };
}

// Pure functions.
verified!(
    "fn_0088bcd0",
    "fn_008a6a00",
    "fn_008aadb0",
    "fn_008d5180",
    "fn_008d70e0",
    "fn_0091b3c0",
    "fn_00925e50",
    "fn_009253f0",
    "fn_0094b8c0",
    "fn_00952630",
    "fn_009529e0",
    "fn_009532a0",
    "fn_009535d0",
    "fn_00953640",
    "fn_0097b490",
    "fn_009b7600",
    "fn_009f62c0",
    "fn_00a71cf0",
    "fn_00ab6f50",
    "fn_00b31650",
    "fn_00b79210",
    "fn_00b79250",
    "fn_00be81d0",
    "fn_00d38c10",
    "fn_00d740a0",
    "fn_00d740e0",
);

// Forwarders.
verified!(
    "fn_009815f0",
    "fn_009856a0",
    "fn_00add1a0",
    "fn_00add1c0",
    "fn_00c6e1c0",
    "fn_00ca4e40",
    "fn_00d8c690",
    "fn_00abbda0",
    "fn_00ade7a0",
    "fn_00b05100",
    "fn_00b33d00",
    "fn_00b33d70",
    "fn_00b33de0",
    "fn_00c6dbb0",
    "fn_00adebd0",
    "fn_00a72820",
    "fn_00aba180",
    "fn_009a3ea0",
    "fn_008ac690",
);

// The slot table cluster (globals).
verified!(
    "fn_00952db0",
    "fn_00952e60",
    "fn_00952de0",
    "fn_00953110",
    "fn_00953210",
    "fn_00e63c90",
    "fn_00953900",
    "fn_00953910",
    "fn_00952700",
    "fn_009526d0",
    "fn_00953160",
    "fn_009526b0",
    "fn_00952660",
);
