//! The differential runtime for these shapes: the export macro the verified
//! rewrites are declared with. Mirrors the surface `lf-checker-rt` gives the
//! rewrites (`export!` with the thiscall and cdecl conventions).
//!
//! The routines of these shapes take no callee stubs and no global windows:
//! each test plants its object in memory it owns and passes the address.
//! Test-support code; the lifted crate itself stays free of `unsafe`.

// Test-only runtime: the export macro declares the rewrites' C-convention
// entry points, which the test calls by name.
#![allow(unsafe_code)]

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`.
#[macro_export]
macro_rules! export {
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (cdecl).
        #[unsafe(no_mangle)]
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
}
