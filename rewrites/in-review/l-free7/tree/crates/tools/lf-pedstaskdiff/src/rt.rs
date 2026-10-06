//! The 32-bit differential runtime: checker macros, global cells and callee stubs.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `global`,
//! `relocated`). The proof set touches 25 global words (the shared
//! state block, its copy source, and the reaction classifier's threshold
//! and bounds, one of them a double word) plus one planted code address
//! (the state consumer, passed to the enumeration call). Each global is
//! one 8-byte atomic cell; the atomics give the cells stable addresses,
//! and the differential tests hold one lock across each whole test, so
//! the rewrite's plain reads and writes through them never race.
//! Test-support code: the lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses through scripted globals are inherent
// here. Every access stays inside the test memory the case planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering;

/// The 25 global cells, planted per test. All are 8 bytes so the one
/// double word fits; single words use the low half.
static CELLS: [AtomicU64; 25] = [
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
];

/// The cell index for a file VA.
///
/// # Panics
///
/// When the address is not one of the 25 VAs the proof set reads or
/// writes as globals: a case bug, never a guess.
fn cell_index(file_va: u32) -> usize {
    match file_va {
        // Shared state block: flag, biases, range, gate vector + pad.
        0x0171_BBE0 => 0,
        0x0171_BBE4 => 1,
        0x0171_BBE8 => 2,
        0x0171_BBEC => 3,
        0x0171_BC10 => 4,
        0x0171_BC14 => 5,
        0x0171_BC18 => 6,
        0x0171_BC1C => 7,
        // Work vector.
        0x0171_BF20 => 8,
        0x0171_BF24 => 9,
        0x0171_BF28 => 10,
        // Raw floats.
        0x0171_BF00 => 11,
        0x0171_BF04 => 12,
        0x0171_BF08 => 13,
        0x0171_BF0C => 14,
        0x0171_BF10 => 15,
        0x0171_BF14 => 16,
        0x0171_BF18 => 17,
        0x0171_BF1C => 18,
        // Extra scratch word.
        0x0171_BF2C => 19,
        // Copy source word (also the reaction counter threshold).
        0x0117_35B4 => 20,
        // Reaction classifier bounds.
        0x00FE_8D68 => 21,
        0x00EB_9504 => 22,
        0x00FE_891C => 23,
        0x00EA_87B8 => 24,
        _ => panic!("unexpected global VA {file_va:#x}"),
    }
}

/// The consumer callback file VA, the only planted (non-cell) address.
const CONSUMER_VA: u32 = 0x00CB_7CF0;

/// Planted address for the consumer callback.
static CONSUMER_ADDR: AtomicU32 = AtomicU32::new(0);

/// Plants the relocated address of a file VA.
///
/// Only the consumer callback is planted; every other VA the proof set
/// relocates is a global cell whose address is fixed.
///
/// # Panics
///
/// When the VA is not the consumer callback: a case bug.
pub fn set_relocated(file_va: u32, addr: u32) {
    assert!(
        file_va == CONSUMER_VA,
        "unexpected relocated VA {file_va:#x}"
    );
    CONSUMER_ADDR.store(addr, Ordering::Relaxed);
}

/// File VA to relocated address, mirroring `lf-checker-rt`.
///
/// # Panics
///
/// When the VA is neither a global cell nor the planted consumer: a case bug.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    if file_va == CONSUMER_VA {
        let addr = CONSUMER_ADDR.load(Ordering::Relaxed);
        assert!(addr != 0, "consumer address read before planting");
        return addr;
    }
    &CELLS[cell_index(file_va)] as *const AtomicU64 as usize as u32
}

/// Pointer to the global at a file VA, mirroring
/// `lf-checker-rt::global`.
///
/// # Panics
///
/// When the VA is not a global cell: a case bug.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    &CELLS[cell_index(file_va)] as *const AtomicU64 as *mut T
}

/// Registered stub addresses for callee ids 0..8 (0 when none: a call
/// there panics, which is a case bug). Each test plants the stubs its
/// rewrites call before running.
static CALLEES: [AtomicU32; 8] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
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
