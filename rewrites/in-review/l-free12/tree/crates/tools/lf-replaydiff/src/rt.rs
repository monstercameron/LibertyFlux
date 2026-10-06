//! The 32-bit differential runtime: checker macros, globals and callee stubs.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `global`,
//! `relocated`). The proof set reads and writes seven globals: the four
//! time-base words, the two stamp-publish words, and the game-state word;
//! one relocated address (the notifier hub); and callee ids 1..=3. Each
//! global is one atomic slot; the atomics give the slots stable addresses,
//! and the differential tests hold one lock across each whole test, so the
//! rewrite's plain reads and writes through them never race.
//! Test-support code: the lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses through scripted globals are inherent
// here. Every access stays inside the test memory the case planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

/// The time-base numerator pair (narrow, wide).
static NUM_SLOTS: [AtomicU32; 2] = [AtomicU32::new(0), AtomicU32::new(0)];
/// The time-base denominator pair (narrow, wide).
static DEN_SLOTS: [AtomicU32; 2] = [AtomicU32::new(0), AtomicU32::new(0)];
/// The stamp-publish flag word and stamp word.
static PUB_SLOTS: [AtomicU32; 2] = [AtomicU32::new(0), AtomicU32::new(0)];
/// The game-state word.
static STATE_SLOT: AtomicU32 = AtomicU32::new(0);

/// Address of the global slot for a file VA.
///
/// # Panics
///
/// When the address is not one of the seven globals the proof set reads:
/// a case bug, never a guess.
fn slot_for(file_va: u32) -> *mut u32 {
    match file_va {
        0x0105_c880 => NUM_SLOTS[0].as_ptr(),
        0x0105_c87c => NUM_SLOTS[1].as_ptr(),
        0x0105_c884 => DEN_SLOTS[0].as_ptr(),
        0x0105_c888 => DEN_SLOTS[1].as_ptr(),
        0x011f_70e0 => PUB_SLOTS[0].as_ptr(),
        0x011f_70e4 => PUB_SLOTS[1].as_ptr(),
        0x0103_7720 => STATE_SLOT.as_ptr(),
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
    slot_for(file_va).cast::<T>()
}

/// Plants the word the global at a file VA holds.
///
/// # Panics
///
/// As [`slot_for`].
pub fn set_global(file_va: u32, value: u32) {
    unsafe { slot_for(file_va).write(value) };
}

/// The notifier hub's relocated address, planted per test.
static HUB_SLOT: AtomicU32 = AtomicU32::new(0);

/// File VA to relocated address, mirroring `lf-checker-rt`.
///
/// # Panics
///
/// When the address is not the hub VA the proof set relocates: a case
/// bug, never a guess.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    match file_va {
        0x0103_e498 => HUB_SLOT.load(Ordering::Relaxed),
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// Plants the relocated address of a stamp VA.
///
/// # Panics
///
/// When the address is not the hub VA.
pub fn set_relocated(file_va: u32, addr: u32) {
    match file_va {
        0x0103_e498 => HUB_SLOT.store(addr, Ordering::Relaxed),
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// Registered stub addresses for callee ids 0..4 (0 when none: a call
/// there panics, which is a case bug). Each test plants the stubs its
/// rewrites call before running.
static CALLEES: [AtomicU32; 4] = [
    AtomicU32::new(0),
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
