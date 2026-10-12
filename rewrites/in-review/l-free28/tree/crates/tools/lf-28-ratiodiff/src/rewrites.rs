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

pub use crate::global;

pub mod fn_00E6A7C0 { #![allow(unused_imports)] use super::{global}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e6a7c0.rs")); }
pub mod fn_00E6A860 { #![allow(unused_imports)] use super::{global}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e6a860.rs")); }
pub mod fn_00E6AA50 { #![allow(unused_imports)] use super::{global}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e6aa50.rs")); }
pub mod fn_00E6AAB0 { #![allow(unused_imports)] use super::{global}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e6aab0.rs")); }
pub mod fn_00E6AAF0 { #![allow(unused_imports)] use super::{global}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e6aaf0.rs")); }
pub mod fn_00E6AB10 { #![allow(unused_imports)] use super::{global}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e6ab10.rs")); }
pub mod fn_00E6AB50 { #![allow(unused_imports)] use super::{global}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e6ab50.rs")); }
pub mod fn_00E6AB70 { #![allow(unused_imports)] use super::{global}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e6ab70.rs")); }
pub mod fn_00E6AB90 { #![allow(unused_imports)] use super::{global}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e6ab90.rs")); }
pub mod fn_00E6ABB0 { #![allow(unused_imports)] use super::{global}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e6abb0.rs")); }
