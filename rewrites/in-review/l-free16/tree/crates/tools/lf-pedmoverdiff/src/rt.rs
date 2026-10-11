//! The 32-bit differential runtime: checker macros and callee stubs.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!` and the `callee_*` macros). The mover rewrites reach their
//! callees through numbered slots, and the differential test plants a stub
//! address in each slot before it runs a rewrite; the slot numbers mean
//! different things in different rewrites, so each test program plants its
//! own. Test-support code: the lifted crate itself stays
//! `#![forbid(unsafe_code)]`.

// Test-only runtime: raw callee addresses are inherent here. Every call
// goes to a stub the test planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

/// Registered stub addresses for callee ids 0..8 (0 when none: a call
/// there panics, which is a case bug).
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

/// Plants the stub address that callee `id` calls land on.
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
