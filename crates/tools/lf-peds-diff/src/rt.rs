//! The 32-bit differential runtime: the checker macros and the scripted globals
//! and callees the scenario rewrites run against.
//!
//! Mirrors the surface the checker builds verified rewrites against (`export!`,
//! the `callee_*` macros, `callee_addr`, `relocated`). Two things are answered
//! by the test rather than by the image: a global word the rewrite reads through
//! `relocated` (set by [`set_global`] to the address of a buffer the test owns),
//! and the stub each callee id calls (set by [`set_callee`]).

use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

/// Scripted global words: `(file address, address the test answers with)`.
static GLOBALS: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());

/// Number of callee ids the rewrites use (ids are small integers in each rewrite).
const CALLEE_IDS: usize = 32;

/// Stub address per callee id; zero when none is registered.
static CALLEES: [AtomicU32; CALLEE_IDS] = [const { AtomicU32::new(0) }; CALLEE_IDS];

/// Answers `relocated(file_va)` with `addr` for the rest of the test binary.
///
/// # Panics
///
/// When the test's global table is poisoned by an earlier panic.
pub fn set_global(file_va: u32, addr: u32) {
    let mut table = GLOBALS.lock().expect("globals table");
    table.retain(|(va, _)| *va != file_va);
    table.push((file_va, addr));
}

/// File address to relocated address, mirroring `lf-checker-rt`. Only the globals
/// the test has set are answered; any other is a case bug.
///
/// # Panics
///
/// When the rewrite reads a global the test did not set.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    let table = GLOBALS.lock().expect("globals table");
    match table.iter().find(|(va, _)| *va == file_va) {
        Some((_, addr)) => *addr,
        None => panic!("global {file_va:#x} read by a rewrite was not set by the test"),
    }
}

/// Registers the stub that callee `id` calls.
pub fn set_callee(id: u32, addr: u32) {
    CALLEES[id as usize].store(addr, Ordering::SeqCst);
}

/// Raw stub address for callee `id`: the registered stub, or zero.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    CALLEES[id as usize].load(Ordering::SeqCst)
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
