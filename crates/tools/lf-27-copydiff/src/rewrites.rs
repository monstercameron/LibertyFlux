//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`
//! (32-bit target only). The include paths are relative to this crate's
//! manifest: three levels up to the repository root, then
//! `rewrites/verified/functions/`.

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

pub use crate::{global, relocated};

pub mod fn_00ac6440 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00ac6440.rs"
    ));
}
pub mod fn_00986030 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00986030.rs"
    ));
}
pub mod fn_00cbbee0 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00cbbee0.rs"
    ));
}
pub mod fn_00dcce20 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00dcce20.rs"
    ));
}
