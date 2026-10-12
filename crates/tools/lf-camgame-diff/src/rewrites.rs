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
    unused_variables,
    clippy::all,
    clippy::pedantic
)]

pub use crate::{callee_addr, global, relocated};

pub mod fn_00b05d30 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00b05d30.rs"
    ));
}

pub mod fn_00b09da0 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00b09da0.rs"
    ));
}

pub mod fn_00b09cf0 {
    #![allow(unused_imports)]
    use super::{callee_addr, global};
    use crate::{callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00b09cf0.rs"
    ));
}

pub mod fn_00b07760 {
    #![allow(unused_imports)]
    use super::{callee_addr, global, relocated};
    use crate::{callee_cdecl, callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00b07760.rs"
    ));
}

pub mod fn_00b06d90 {
    #![allow(unused_imports)]
    use super::{callee_addr, global, relocated};
    use crate::{callee_cdecl, callee_thiscall, export};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00b06d90.rs"
    ));
}
