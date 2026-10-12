//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`
//! (32-bit target only). The include paths are relative to this crate's
//! manifest: three levels up to the repository root, then
//! `rewrites/verified/functions/`.

// Byte-identical verified files; their style is not linted here.
#![allow(
    unsafe_code,
    dead_code,
    missing_docs,
    non_snake_case,
    unused_doc_comments,
    unused_imports,
    unused_mut,
    unused_variables,
    clippy::all,
    clippy::pedantic
)]

pub use crate::callee_addr;
pub use crate::global;
pub use crate::relocated;

// One module per verified rewrite: files share top-level helper names, so
// each lives in its own scope. Files that use the bare macro names import
// them here.

pub mod fn_0088F700 {
    #![allow(unused_imports)]
    use super::{callee_addr, global, relocated};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_0088f700.rs"
    ));
}

pub mod fn_0088F7C0 {
    #![allow(unused_imports)]
    use super::{callee_addr, global, relocated};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_0088f7c0.rs"
    ));
}

pub mod fn_00890800 {
    #![allow(unused_imports)]
    use super::{callee_addr, global, relocated};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00890800.rs"
    ));
}

pub mod fn_00890870 {
    #![allow(unused_imports)]
    use super::{callee_addr, global, relocated};
    use crate::{callee_cdecl, callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00890870.rs"
    ));
}

pub mod fn_00891680 {
    #![allow(unused_imports)]
    use super::{callee_addr, global, relocated};
    use crate::{callee_cdecl, callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00891680.rs"
    ));
}

pub mod fn_008916A0 {
    #![allow(unused_imports)]
    use super::{callee_addr, global, relocated};
    use crate::{callee_cdecl, callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_008916a0.rs"
    ));
}

pub mod fn_00891E90 {
    #![allow(unused_imports)]
    use super::{callee_addr, global, relocated};
    use crate::{callee_cdecl, callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00891e90.rs"
    ));
}

pub mod fn_008909C0 {
    #![allow(unused_imports)]
    use super::{callee_addr, global, relocated};
    use crate::{callee_cdecl, callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_008909c0.rs"
    ));
}

pub mod fn_0088FFF0 {
    #![allow(unused_imports)]
    use super::{callee_addr, global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_0088fff0.rs"
    ));
}
