//! `lf-checker-rt`: tiny runtime for checker rewrites (zero dependencies).
//!
//! Usage: rewrite crates declare each export with [`export!`] in the
//! original's calling convention, derive every address from
//! [`relocated`]/[`global`] (never hard-code a mapped address), and call
//! intercepted callees through the `callee_*` macros. The worker patches
//! the data exports below after loading the rewrite DLL: the relocated
//! image base, the callee stub table, and the XMM/TLS mirrors.
//!
//! Rewrites are plain `#[unsafe(no_mangle)] pub extern "<conv>" fn` items. The
//! `export!` macro keeps the declaration uniform; the `callee_*` macros
//! call intercepted callees through the worker-owned stub table.

// The worker patches the statics below through raw pointers and every
// accessor dereferences worker-owned memory, so unsafe is inherent here.
#![allow(unsafe_code)]
// Integrated lane code, proven by the checker's regression suite; pedantic
// style lints stay off here while correctness lints (clippy::all) apply.
#![allow(clippy::pedantic)]
#![no_std]

/// Relocated base of the mapped original image, patched by the worker.
/// Reads 0 until the worker loads this DLL.
#[unsafe(no_mangle)]
pub static mut CHECKER_XBASE: u32 = 0;

/// Pointer to the worker's callee stub table (256 stub addresses, 0 for
/// undeclared ids), patched by the worker.
#[unsafe(no_mangle)]
pub static mut CHECKER_CTABLE: *const u32 = core::ptr::null();

/// Pointer to the worker's mirror of the trial's scripted XMM entry values
/// (32 words: xmm0-7 low-to-high), patched by the worker. A Rust rewrite
/// cannot observe incoming vector registers any other way; the original
/// reads the same values from its registers, so equality of the values is
/// still verified, only the transport differs.
#[unsafe(no_mangle)]
pub static mut CHECKER_XMM: *const u32 = core::ptr::null();

/// One word of the trial's scripted XMM entry state: register `reg` (0-7),
/// word `i` (0-3, low to high).
#[inline(always)]
#[must_use]
pub fn xmm_word(reg: usize, i: usize) -> u32 {
    // SAFETY: the worker sets CHECKER_XMM to its 32-word mirror before any
    // trial runs; reg < 8 and i < 4 are the caller's contract.
    unsafe {
        core::ptr::addr_of!(CHECKER_XMM)
            .read()
            .add(reg * 4 + i)
            .read()
    }
}

/// Pointer to the worker's mirror of the trial's fabricated TLS slot values
/// (256 words), patched by the worker. The original reads the same values
/// through FS:[0x2c]; the rewrite reads them here.
#[unsafe(no_mangle)]
pub static mut CHECKER_TLS: *const u32 = core::ptr::null();

/// The trial's fabricated value of TLS slot `slot` (0-63).
#[inline(always)]
#[must_use]
pub fn tls_slot(slot: usize) -> u32 {
    // SAFETY: the worker sets CHECKER_TLS to its 256-word mirror before any
    // trial runs; slot < 64 is the caller's contract.
    unsafe { core::ptr::addr_of!(CHECKER_TLS).read().add(slot).read() }
}

/// Relocated base of the original image.
#[inline(always)]
#[must_use]
pub fn xbase() -> u32 {
    // SAFETY: the worker patches CHECKER_XBASE at load; it is read-only after.
    unsafe { core::ptr::addr_of!(CHECKER_XBASE).read() }
}

/// Convert a file VA (as seen in disassembly, image base 0x400000) to the
/// relocated address in the worker's mapping.
#[inline(always)]
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    file_va.wrapping_sub(0x400000).wrapping_add(xbase())
}

/// Pointer to a global at a file VA.
#[inline(always)]
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    relocated(file_va) as *mut T
}

/// Raw stub address for callee `id` (0 when undeclared: calling it faults,
/// which the worker reports as a rewrite failure, never silently).
#[inline(always)]
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    // SAFETY: the worker sets CHECKER_CTABLE to its 256-entry stub table
    // before any trial runs; ids below 256 are the caller's contract.
    unsafe {
        let table = core::ptr::addr_of!(CHECKER_CTABLE).read();
        if table.is_null() {
            return 0;
        }
        table.add(id as usize).read()
    }
}

/// Declare a rewrite export with the original's calling convention.
///
/// ```ignore
/// lf_checker_rt::export!(cdecl, my_rewrite(a: u32, b: u32) -> u32 {
///     a.wrapping_add(b)
/// });
/// ```
/// Conventions: `cdecl`, `stdcall`, `thiscall`, `fastcall`. The first
/// parameter of a `thiscall`/`fastcall` is passed in ECX (and the second of a
/// `fastcall` in EDX), matching 32-bit MSVC.
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the checker worker by name (cdecl).
        #[unsafe(no_mangle)]
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
    (stdcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the checker worker by name (stdcall).
        #[unsafe(no_mangle)]
        pub extern "stdcall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the checker worker by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (fastcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the checker worker by name (fastcall).
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

/// Call intercepted callee `id` with the thiscall convention: the first
/// argument is passed in ECX, the rest on the stack, callee cleans up.
/// (Folded from pilot lanes q-07/q-08.)
#[macro_export]
macro_rules! callee_thiscall {
    ($id:expr, $ret:ty, $this_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "thiscall" fn(u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($this_arg $(, $arg )*)
    }};
}

/// Call intercepted callee `id` with the fastcall convention: the first two
/// arguments are passed in ECX and EDX, the rest on the stack, callee cleans.
/// (Folded from the pilot lanes; proven by the checker's tail-thunk proofs.)
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
