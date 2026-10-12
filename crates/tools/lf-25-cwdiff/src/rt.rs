//! The 32-bit runtime for the constant-write batch.
//!
//! Mirrors the surface the checker builds verified rewrites against (`export!`,
//! `global`, `relocated`). These rewrites take their object as an argument and
//! do not touch globals; relocation is the identity here, and the lifted routine
//! is given the same constants.

/// Mirrors `lf-checker-rt::relocated`. Identity in this runtime.
#[must_use]
pub const fn relocated(file_va: u32) -> u32 {
    file_va
}

/// Mirrors `lf-checker-rt::global`. No global is touched by this batch.
///
/// # Panics
///
/// Always: a routine of this batch that reaches a global is a test bug.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    panic!("test bug: the constant-write batch reads no global ({file_va:#x})")
}

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`.
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (cdecl).
        #[unsafe(no_mangle)]
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}
