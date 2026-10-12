//! `lf-soundbase-diff`: differential tests of the lifted base sound class.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` via `extern crate self as`, with
//! the same macro and function names the checker builds them against. The
//! lifted methods run against them on the same generated inputs, in one
//! test memory arena at real 32-bit addresses, comparing results, every
//! call with its arguments in order, and every byte of the arena.
//!
//! The differential cases run on the 32-bit target only: the rewrites take
//! real addresses. On other hosts the crate builds and runs no rewrite case
//! (the lifted crate's own host tests run everywhere).

// The crate root re-exports the runtime surface under the checker's names
// so `use lf_checker_rt::{...}` and `lf_checker_rt::...` paths in the
// included verified files resolve here.
extern crate self as lf_checker_rt;

// The runtime declares stand-ins with the 32-bit calling conventions the
// rewrites call through, which exist only on that target.
#[cfg(target_arch = "x86")]
pub mod rt;

#[cfg(target_arch = "x86")]
pub use rt::callee_addr;
#[cfg(target_arch = "x86")]
pub use rt::global;
#[cfg(target_arch = "x86")]
pub use rt::relocated;

/// Declare a rewrite export with the original's calling convention. Mirrors
/// `lf-checker-rt::export`; the rewrites reach it through `lf_checker_rt`.
#[cfg(target_arch = "x86")]
#[macro_export]
macro_rules! export {
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}

/// Call intercepted callee `id` with the cdecl convention.
#[cfg(target_arch = "x86")]
#[macro_export]
macro_rules! callee_cdecl {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        #[allow(unused_unsafe)]
        let f: extern "cdecl" fn($( $crate::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($( $arg ),*)
    }};
}

/// Call intercepted callee `id` with the thiscall convention.
#[cfg(target_arch = "x86")]
#[macro_export]
macro_rules! callee_thiscall {
    ($id:expr, $ret:ty, $this_arg:expr $(, $arg:expr)* $(,)?) => {{
        #[allow(unused_unsafe)]
        let f: extern "thiscall" fn(u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($this_arg $(, $arg )*)
    }};
}

#[doc(hidden)]
#[cfg(target_arch = "x86")]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}

#[cfg(target_arch = "x86")]
pub mod rewrites;
