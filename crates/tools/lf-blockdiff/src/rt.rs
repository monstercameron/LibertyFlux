//! The 32-bit differential runtime for the row-lookup rewrites.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`). The row lookups read and write only the object they are given,
//! so the runtime needs no global memory: the test passes the object's address
//! as `this`.

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`, for the thiscall convention used by the
/// row-lookup family.
#[macro_export]
macro_rules! export {
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}
