//! The 32-bit differential runtime: checker macros and shared constants.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). The proof set is call-free, so the callee table stays
//! empty; what the samplers need is `global`, answered from the measured
//! constant values. Test-support code: the lifted crate itself stays
//! `#![forbid(unsafe_code)]`.

// Test-only runtime: raw pointers through scripted addresses are inherent
// here. Every access stays inside the test image the case built.
#![allow(unsafe_code)]

/// The shared rounding half, measured from the executable's data.
static ROUND_HALF_BITS: u32 = 0x3F00_0000;
/// The round-to-nearest bias (2^23), measured the same way.
static ROUND_MAGIC_BITS: u32 = 0x4B00_0000;
/// One, measured the same way.
static ONE_BITS: u32 = 0x3F80_0000;
/// The snap-down threshold, measured the same way.
static SNAP_LO_BITS: u32 = 0x3A83_126F;
/// The snap-up threshold, measured the same way.
static SNAP_HI_BITS: u32 = 0x3F7F_BE77;

/// Pointer to the shared constant at a file VA, mirroring
/// `lf-checker-rt::global` (image base `0x400000`).
///
/// # Panics
///
/// When the address is not one of the five constants the proof set
/// reads: a case bug, never a guess.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    let word: *const u32 = match file_va {
        0xFE86B4 => &SNAP_LO_BITS,
        0xFE88DC => &SNAP_HI_BITS,
        0xFE88E8 => &ONE_BITS,
        0xFE8CF8 => &ROUND_MAGIC_BITS,
        0xFE8830 => &ROUND_HALF_BITS,
        _ => panic!("unexpected shared constant VA {file_va:#x}"),
    };
    word as *mut T
}

/// Relocated base, mirroring `lf-checker-rt` (unused by the proof set:
/// none of its files relocates an address, but the name must resolve).
pub static mut CHECKER_XBASE: u32 = 0;

/// File VA to relocated address, mirroring `lf-checker-rt`.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    let xbase = unsafe { core::ptr::addr_of!(CHECKER_XBASE).read() };
    file_va.wrapping_sub(0x400000).wrapping_add(xbase)
}

/// Raw stub address for callee `id` (always 0: the proof set makes no
/// callee calls, but the name must resolve).
#[must_use]
pub const fn callee_addr(_id: u32) -> u32 {
    0
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
    (stdcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (stdcall).
        #[unsafe(no_mangle)]
        pub extern "stdcall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (fastcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (fastcall).
        #[unsafe(no_mangle)]
        pub extern "fastcall" fn $name($($arg : $ty),*) -> $ret $body
    };
}

/// Call intercepted callee `id` with the cdecl convention.
#[macro_export]
macro_rules! callee_cdecl {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        let f: extern "cdecl" fn($( $crate::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($( $arg ),*)
    }};
}

/// Call intercepted callee `id` with the stdcall convention.
#[macro_export]
macro_rules! callee_stdcall {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        let f: extern "stdcall" fn($( $crate::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($( $arg ),*)
    }};
}

/// Call intercepted callee `id` with the thiscall convention.
#[macro_export]
macro_rules! callee_thiscall {
    ($id:expr, $ret:ty, $this_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "thiscall" fn(u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($this_arg $(, $arg )*)
    }};
}

/// Call intercepted callee `id` with the fastcall convention.
#[macro_export]
macro_rules! callee_fastcall {
    ($id:expr, $ret:ty, $ecx_arg:expr, $edx_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "fastcall" fn(u32, u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($ecx_arg, $edx_arg $(, $arg )*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
