//! The 32-bit differential runtime: checker macros and callee stubs.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_thiscall!` macro, `callee_addr`,
//! `relocated`). The proof set reads one relocated word: the shared
//! header word the kind initialisers forward to the main routine. It
//! lives in one atomic slot holding the planted value; the atomics give
//! the slot a stable address, and the differential tests hold one lock
//! across each whole test, so the rewrites' plain reads through it
//! never race. The kind-`0x3b` float constants live in two more
//! slots of the same shape.
//! Test-support code: the lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses through scripted globals are inherent
// here. Every access stays inside the test memory the case planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

/// The shared header word's value, planted per case.
static GG_WORD: AtomicU32 = AtomicU32::new(0);

/// Plants the shared header word's value.
pub fn set_gg(v: u32) {
    GG_WORD.store(v, Ordering::Relaxed);
}

/// The two shared float constants' values, planted per case.
static F1_WORD: AtomicU32 = AtomicU32::new(0);
static F2_WORD: AtomicU32 = AtomicU32::new(0);

/// Plants the first shared float constant's bits.
pub fn set_f1(v: u32) {
    F1_WORD.store(v, Ordering::Relaxed);
}

/// Plants the second shared float constant's bits.
pub fn set_f2(v: u32) {
    F2_WORD.store(v, Ordering::Relaxed);
}

/// File VA to relocated address, mirroring `lf-checker-rt`.
///
/// The shared header word's VA maps to the planted slot's address; the
/// rewrites read the value through it.
///
/// # Panics
///
/// When the address is not the shared header word's VA: a case bug,
/// never a guess.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    let ptr = match file_va {
        0x0117_35A4 => GG_WORD.as_ptr(),
        0x0139_C25C => F1_WORD.as_ptr(),
        0x00FE_8A94 => F2_WORD.as_ptr(),
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    };
    u32::try_from(ptr as usize).expect("relocated word address fits in 32 bits")
}

/// Registered stub addresses for callee ids 0..5 (0 when none: a call
/// there panics, which is a case bug). Each test clears the table, then
/// plants the stubs its rewrite calls before running.
static CALLEES: [AtomicU32; 5] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

/// Clears every callee slot, so a stale plant from an earlier test can
/// never mask a call the current test did not plant for.
pub fn clear_callees() {
    for slot in &CALLEES {
        slot.store(0, Ordering::Relaxed);
    }
}

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
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
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
