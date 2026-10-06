//! The 32-bit differential runtime: checker macros, globals and callee stubs.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `global`,
//! `relocated`). The proof set relocates four file VAs: the data-slot
//! region base, the kind-flag region base (11 bytes into the same
//! region), the slot-index kind table base, and one cell holding the
//! entry-array base; one thiscall callee slot (the location routine's
//! address callee) is replanted per test. Each global is one atomic
//! slot; the atomics give the slots stable addresses, and the
//! differential tests hold one lock across each whole test, so the
//! rewrite's plain reads and writes through them never race.
//! Test-support code: the lifted crate itself denies `unsafe_code`.

// Test-only runtime: raw addresses through scripted globals are inherent
// here. Every access stays inside the test memory the case planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

/// The data-slot region base (file VA `0x012fb44c`).
static DATA_SLOT: AtomicU32 = AtomicU32::new(0);
/// The kind-flag region base (file VA `0x012fb457`).
static KIND_FLAG_SLOT: AtomicU32 = AtomicU32::new(0);
/// The slot-index kind table base (file VA `0x013053a8`).
static KIND_TABLE_SLOT: AtomicU32 = AtomicU32::new(0);
/// The cell holding the entry-array base (file VA `0x012fb3a8`).
static TABLE_BASE_SLOT: AtomicU32 = AtomicU32::new(0);

/// The relocated slot for a file VA.
///
/// # Panics
///
/// When the address is not one of the four VAs the proof set relocates:
/// a case bug, never a guess.
fn reloc_slot(file_va: u32) -> &'static AtomicU32 {
    match file_va {
        0x012f_b44c => &DATA_SLOT,
        0x012f_b457 => &KIND_FLAG_SLOT,
        0x0130_53a8 => &KIND_TABLE_SLOT,
        0x012f_b3a8 => &TABLE_BASE_SLOT,
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// Plants the relocated address of a file VA.
pub fn set_relocated(file_va: u32, addr: u32) {
    reloc_slot(file_va).store(addr, Ordering::Relaxed);
}

/// File VA to relocated address, mirroring `lf-checker-rt`.
///
/// # Panics
///
/// When the address is not one of the four VAs the proof set relocates.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    reloc_slot(file_va).load(Ordering::Relaxed)
}

/// Address of the global slot for a file VA.
///
/// The proof set's rewrites use `relocated` only; this mirrors
/// `lf-checker-rt::global` for the one `global` import the shared
/// include prelude carries.
///
/// # Panics
///
/// Always: no proof case should reach it.
fn slot_for(file_va: u32) -> *mut u32 {
    panic!("unexpected global VA {file_va:#x}")
}

/// Pointer to the global at a file VA, mirroring
/// `lf-checker-rt::global`.
///
/// # Panics
///
/// Always (see [`slot_for`]).
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    slot_for(file_va) as *mut T
}

/// Registered stub address for callee id 0 (0 when none: a call there
/// panics, which is a case bug). Each test plants the stub its rewrite
/// calls before running.
static CALLEE0: AtomicU32 = AtomicU32::new(0);

/// Plants the stub address callee `id` calls land on.
///
/// # Panics
///
/// When `id` is not 0: the proof set calls only callee 0.
pub fn set_callee(id: u32, addr: u32) {
    assert!(id == 0, "unexpected callee id {id}");
    CALLEE0.store(addr, Ordering::Relaxed);
}

/// Raw stub address for callee `id`.
///
/// # Panics
///
/// When `id` is not 0 or no stub is planted: a case bug.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    assert!(id == 0, "unexpected callee id {id}");
    let addr = CALLEE0.load(Ordering::Relaxed);
    assert!(addr != 0, "callee {id} called with no stub planted");
    addr
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
