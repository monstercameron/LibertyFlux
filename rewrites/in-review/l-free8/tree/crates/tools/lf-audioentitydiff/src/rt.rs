//! The 32-bit differential runtime: checker macros, globals and callee stubs.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `global`,
//! `relocated`). The proof set reads five globals (the playback mode,
//! current id and three alternate ids) and three relocated addresses
//! (the latch configuration word and the constructor's two table
//! words). Each global is one atomic slot; the atomics give the slots
//! stable addresses, and the differential tests hold one lock across
//! each whole test, so the rewrite's plain reads and writes through
//! them never race.
//! Test-support code: the lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses through scripted globals are inherent
// here. Every access stays inside the test memory the case planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

/// The playback mode word.
static MODE_SLOT: AtomicU32 = AtomicU32::new(0);
/// The current entity id word.
static CUR_SLOT: AtomicU32 = AtomicU32::new(0);
/// The three alternate id words.
static ALT_SLOTS: [AtomicU32; 3] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

/// Address of the global slot for a file VA.
///
/// # Panics
///
/// When the address is not one of the five globals the proof set reads:
/// a case bug, never a guess.
fn slot_for(file_va: u32) -> *mut u32 {
    match file_va {
        0x0179_BF98 => MODE_SLOT.as_ptr(),
        0x0179_BFA0 => CUR_SLOT.as_ptr(),
        0x0179_BFA4 => ALT_SLOTS[0].as_ptr(),
        0x0179_BFA8 => ALT_SLOTS[1].as_ptr(),
        0x0179_BFAC => ALT_SLOTS[2].as_ptr(),
        _ => panic!("unexpected global VA {file_va:#x}"),
    }
}

/// Pointer to the global at a file VA, mirroring
/// `lf-checker-rt::global`.
///
/// # Panics
///
/// As [`slot_for`].
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    slot_for(file_va) as *mut T
}

/// The relocated addresses, planted per test: the latch configuration
/// word, then the constructor's two table words.
static RELOC_SLOTS: [AtomicU32; 3] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

/// The slot for a file VA.
///
/// # Panics
///
/// When the address is not one of the three VAs the proof set
/// relocates: a case bug, never a guess.
fn reloc_slot(file_va: u32) -> &'static AtomicU32 {
    match file_va {
        0x0117_35B4 => &RELOC_SLOTS[0],
        0x00E9_1480 => &RELOC_SLOTS[1],
        0x00E9_08AC => &RELOC_SLOTS[2],
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// Plants the relocated address of a VA.
pub fn set_relocated(file_va: u32, addr: u32) {
    reloc_slot(file_va).store(addr, Ordering::Relaxed);
}

/// File VA to relocated address, mirroring `lf-checker-rt`.
///
/// # Panics
///
/// As [`reloc_slot`].
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    reloc_slot(file_va).load(Ordering::Relaxed)
}

/// Registered stub addresses for callee ids 0..3 (0 when none: a call
/// there panics, which is a case bug). Each test plants the stubs its
/// rewrites call before running.
static CALLEES: [AtomicU32; 3] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

/// Plants the stub address callee `id` calls land on.
pub fn set_callee(id: u32, addr: u32) {
    CALLEES[id as usize].store(addr, Ordering::Relaxed);
}

/// Raw stub address for callee `id`.
///
/// # Panics
///
/// When `id` is outside the table or no stub is planted: a case bug.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    let slot = CALLEES
        .get(id as usize)
        .unwrap_or_else(|| panic!("unexpected callee id {id}"));
    let addr = slot.load(Ordering::Relaxed);
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

/// Call intercepted callee `id` with the thiscall convention.
#[macro_export]
macro_rules! callee_thiscall {
    ($id:expr, $ret:ty, $this_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "thiscall" fn(u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($this_arg $(, $arg )*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
