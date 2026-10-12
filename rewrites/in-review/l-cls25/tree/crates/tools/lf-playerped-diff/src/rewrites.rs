//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`
//! (32-bit target only). The include paths are relative to this crate's
//! manifest: three levels up to the repository root, then
//! `rewrites/verified/functions/`.

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

pub use crate::{callee_addr, global};

pub mod fn_009f0fc0 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_009f0fc0.rs"
    ));
}

pub mod fn_00a29e40 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00a29e40.rs"
    ));
}

pub mod fn_009eaee0 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_009eaee0.rs"
    ));
}

pub mod fn_009e5940 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_009e5940.rs"
    ));
}

pub mod fn_009e54c0 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_cdecl, callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_009e54c0.rs"
    ));
}

pub mod fn_00a2a620 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_cdecl, callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00a2a620.rs"
    ));
}

pub mod fn_00a2d300 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_cdecl, callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00a2d300.rs"
    ));
}

pub mod fn_00a2d3e0 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00a2d3e0.rs"
    ));
}

pub mod fn_00a2d3a0 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00a2d3a0.rs"
    ));
}

pub mod fn_009edb70 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_009edb70.rs"
    ));
}

pub mod fn_00a2a870 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_cdecl, callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00a2a870.rs"
    ));
}
