//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged. The include paths are relative to this
//! crate's manifest: three levels up to the repository root, then
//! `rewrites/verified/functions/`. Each file is its own module, because the
//! files share top-level helper names. Every module imports the export macro
//! from the shared checker runtime, which is what the files are written
//! against.
//!
//! Only the rewrites of methods that call no other code are here: the
//! methods that call out wait for a callee harness.

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

/// Verified rewrite of the word setter at 0xDC.
pub mod fn_00A64D30 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00a64d30.rs"));
}

/// Verified rewrite of the word setter at 0xD4.
pub mod fn_00A64D50 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00a64d50.rs"));
}

/// Verified rewrite of the flag setter at 0xC8.
pub mod fn_00C0F4F0 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f4f0.rs"));
}

/// Verified rewrite of the flag setter at 0xC4.
pub mod fn_00C0F500 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f500.rs"));
}

/// Verified rewrite of the float setter at 0x8.
pub mod fn_00C0F510 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f510.rs"));
}

/// Verified rewrite of the float setter at 0x4.
pub mod fn_00C0F520 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f520.rs"));
}

/// Verified rewrite of the flag setter at 0xD1.
pub mod fn_00C0F530 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f530.rs"));
}

/// Verified rewrite of the float store that clears the tag words at 0xB4 and 0xB8.
pub mod fn_00C0F540 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f540.rs"));
}

/// Verified rewrite of the float store with a tag compare at 0xB4.
pub mod fn_00C0F570 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f570.rs"));
}

/// Verified rewrite of the flag setter at 0xC7.
pub mod fn_00C0F5A0 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f5a0.rs"));
}

/// Verified rewrite of the flag setter at 0xD0.
pub mod fn_00C0F5B0 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f5b0.rs"));
}

/// Verified rewrite of the word setter at 0xE0.
pub mod fn_00C0F830 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f830.rs"));
}

/// Verified rewrite of the flag setter at 0xEB.
pub mod fn_00C0F840 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f840.rs"));
}

/// Verified rewrite of the word setter at 0xE4.
pub mod fn_00C0F850 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f850.rs"));
}

/// Verified rewrite of the word setter at 0xD8.
pub mod fn_00C0F860 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f860.rs"));
}

/// Verified rewrite of the 24-byte block copy into 0x10.
pub mod fn_00C0F890 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f890.rs"));
}

/// Verified rewrite of the word setter at 0xA0.
pub mod fn_00C0F8C0 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f8c0.rs"));
}

/// Verified rewrite of the flag setter at 0xCA.
pub mod fn_00C0F8D0 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f8d0.rs"));
}

/// Verified rewrite of the 24-byte block copy into 0xA4.
pub mod fn_00C0F8F0 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00c0f8f0.rs"));
}

/// Verified rewrite of the flag setter at 0xE8.
pub mod fn_00DB1940 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00db1940.rs"));
}

/// Verified rewrite of the flag setter at 0xE9.
pub mod fn_00DB1950 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00db1950.rs"));
}

/// Verified rewrite of the flag setter at 0xEA.
pub mod fn_00DB1960 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00db1960.rs"));
}

/// Verified rewrite of the flag setter at 0xC9.
pub mod fn_00DB1970 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00db1970.rs"));
}

/// Verified rewrite of the integer and float slot pair at 0x14 and 0x18.
pub mod fn_00DB1980 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00db1980.rs"));
}

/// Verified rewrite of the cache install at 0xEC.
pub mod fn_00DB1AA0 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00db1aa0.rs"));
}

/// Verified rewrite of the token follow-up that enables the frame twice (token at 0xD4).
pub mod fn_00DB16B0 {
    #![allow(unused_imports)]
    use lf_checker_rt::{callee_cdecl, export};
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00db16b0.rs"));
}

/// Verified rewrite of the token follow-up that resets the frame (token at 0xD4).
pub mod fn_00DB1700 {
    #![allow(unused_imports)]
    use lf_checker_rt::{callee_cdecl, export};
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00db1700.rs"));
}

/// Verified rewrite of the token follow-up that answers the target's step (token at 0xD8).
pub mod fn_00DB1890 {
    #![allow(unused_imports)]
    use lf_checker_rt::{callee_cdecl, export};
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00db1890.rs"));
}