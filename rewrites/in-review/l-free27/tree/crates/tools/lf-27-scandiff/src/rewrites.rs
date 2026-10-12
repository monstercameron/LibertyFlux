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

pub mod fn_00a4ab10 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00a4ab10.rs")); }
pub mod fn_00b50d10 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00b50d10.rs")); }
pub mod fn_00a63780 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00a63780.rs")); }
pub mod fn_00cd15c0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00cd15c0.rs")); }
pub mod fn_00a45490 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00a45490.rs")); }
pub mod fn_00d28bd0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00d28bd0.rs")); }
pub mod fn_00ca2380 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00ca2380.rs")); }
pub mod fn_008ece40 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_008ece40.rs")); }
