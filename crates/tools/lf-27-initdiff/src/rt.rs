//! The 32-bit differential runtime: the checker's names for the export macro,
//! globals and relocation.
//!
//! The rewrites of this family read only the object they are given, so no
//! global is ever touched: [`global`] is an error if a rewrite reaches for
//! one. A relocated address is the file address itself in this runtime.

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export` for the thiscall form these rewrites use.
#[macro_export]
macro_rules! export {
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}

/// A global word of the original. This family has none: a rewrite that asks
/// for one is outside the family, and the test stops rather than guess.
#[must_use]
#[allow(dead_code)]
pub fn global<T>(file_va: u32) -> *mut T {
    panic!("the counted-run family reads no global (asked for {file_va:#x})")
}

/// The relocated address of a file VA. This runtime maps a file VA to itself.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    file_va
}
