// One module per verified rewrite: files share top-level helper names,
// so they cannot live in one scope. Files using bare macros import them
// from the crate root; the rest qualify them.
pub mod fn_006620e0 { #![allow(unused_imports)] use crate::{export, callee_cdecl}; use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_006620e0.rs")); }
pub mod fn_006621c0 { #![allow(unused_imports)] use crate::{export}; use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_006621c0.rs")); }
pub mod fn_00662200 { #![allow(unused_imports)] use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00662200.rs")); }
pub mod fn_00662440 { #![allow(unused_imports)] use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00662440.rs")); }
