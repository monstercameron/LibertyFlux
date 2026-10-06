//! The 32-bit differential runtime: checker macros, the shared manager
//! global and the callee stub registry.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). The call-free proof set uses `export!` only (its virtual
//! calls go through fake tables the cases plant); the factory-pair slots
//! read the shared manager global and call callee slots 1 and 2, which
//! each factory test binary registers before running (one binary per
//! slot, so the registry cells are never shared between tests).
//! Test-support code: the lifted crate itself stays
//! `#![forbid(unsafe_code)]`.

// Test-only runtime: raw pointers through scripted addresses are inherent
// here. Every access stays inside the test image the case built.
#![allow(unsafe_code)]

use core::sync::atomic::{AtomicU32, Ordering};

/// File VA of the shared factory-manager word.
const MANAGER_VA: u32 = 0x0167E2A0;

/// The shared manager word the factory-pair slots read.
static MANAGER: AtomicU32 = AtomicU32::new(0);

/// Sets the shared manager word for the next rewrite call.
pub fn set_manager(word: u32) {
    MANAGER.store(word, Ordering::SeqCst);
}

/// Pointer to the global at a file VA, mirroring
/// `lf-checker-rt::global`.
///
/// # Panics
///
/// When the address is not the shared manager word: the proof set
/// reads nothing else, so any other call is a case bug.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    assert!(
        file_va == MANAGER_VA,
        "unexpected global VA {file_va:#x}: the proof set reads the manager word only"
    );
    core::ptr::addr_of!(MANAGER) as *mut T
}

/// File VA to relocated address, mirroring `lf-checker-rt`.
///
/// # Panics
///
/// Always: the proof set relocates nothing, so any call is a case bug.
#[must_use]
pub fn relocated(_file_va: u32) -> u32 {
    panic!("unexpected relocated() call: the proof set relocates nothing");
}

/// Registered stub addresses for callee slots 1 and 2 (0 until the
/// test binary registers its stubs: a call there then faults, which is
/// a case bug). One test binary per factory slot, so no binary mixes
/// two callees.
static CALLEE1: AtomicU32 = AtomicU32::new(0);
static CALLEE2: AtomicU32 = AtomicU32::new(0);

/// Registers the stub address callee 1 calls land on.
pub fn set_callee1(addr: u32) {
    CALLEE1.store(addr, Ordering::SeqCst);
}

/// Registers the stub address callee 2 calls land on.
pub fn set_callee2(addr: u32) {
    CALLEE2.store(addr, Ordering::SeqCst);
}

/// Raw stub address for callee `id`.
///
/// # Panics
///
/// When the id is not 1 or 2: the factory proof set uses those two
/// slots only.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    match id {
        1 => CALLEE1.load(Ordering::SeqCst),
        2 => CALLEE2.load(Ordering::SeqCst),
        _ => panic!("unexpected callee id {id}: the proof set uses slots 1 and 2 only"),
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
    }
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
