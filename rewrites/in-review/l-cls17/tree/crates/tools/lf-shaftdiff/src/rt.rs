//! The 32-bit differential runtime: checker macros and the shared constants.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, `callee_cdecl!`, `callee_addr`, `global`). The four float
//! constants the shaft rewrites read are answered from the values measured
//! from the executable's data. One callee is registered per test binary.

// Test-only runtime: rewrites are called through raw addresses and read
// the constants through raw pointers. Every access stays inside test data.
#![allow(unsafe_code)]

use core::sync::atomic::{AtomicU32, Ordering};

/// The constant two (measured from the executable's data).
static TWO_BITS: u32 = 0x4000_0000;
/// The constant one half (measured from the executable's data).
static HALF_BITS: u32 = 0x3F00_0000;
/// The constant one (measured from the executable's data).
static ONE_BITS: u32 = 0x3F80_0000;
/// The constant minus one (measured from the executable's data).
static NEG_ONE_BITS: u32 = 0xBF80_0000;

/// The shared constant at a file address, if it is one the shaft rewrites
/// read.
fn const_bits(file_va: u32) -> Option<*const u32> {
    match file_va {
        0xFE8A24 => Some(&TWO_BITS),
        0xFE8830 => Some(&HALF_BITS),
        0xFE88E8 => Some(&ONE_BITS),
        0xFE8D94 => Some(&NEG_ONE_BITS),
        _ => None,
    }
}

/// Pointer to the shared constant at a file address, mirroring
/// `lf-checker-rt::global` (image base `0x400000`).
///
/// # Panics
///
/// When the address is not one of the constants the shaft rewrites read:
/// a case bug, never a guess.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    let word =
        const_bits(file_va).unwrap_or_else(|| panic!("unexpected shared constant address {file_va:#x}"));
    word as *mut T
}

/// Registered stub address for callee 1 (0 when none: a call there then
/// faults, which is a case bug).
static CALLEE1: AtomicU32 = AtomicU32::new(0);

/// Registers the stub address callee 1 (the square-root helper) calls
/// land on.
pub fn set_callee1(addr: u32) {
    CALLEE1.store(addr, Ordering::SeqCst);
}

/// Raw stub address for callee `id`: the registered stub for id 1, 0
/// otherwise.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    if id == 1 {
        CALLEE1.load(Ordering::SeqCst)
    } else {
        0
    }
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

/// Call intercepted callee `id` with the cdecl convention.
#[macro_export]
macro_rules! callee_cdecl {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        let f: extern "cdecl" fn($( $crate::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($( $arg ),*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
