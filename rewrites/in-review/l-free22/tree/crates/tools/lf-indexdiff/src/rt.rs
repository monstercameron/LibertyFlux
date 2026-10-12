//! The export macro of the differential runtime.
//!
//! Mirrors the checker's `export!` surface. Each arm declares the rewrite with the original's calling
//! convention, so the test calls it the same way the checker does.

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`.
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (cdecl).
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}
