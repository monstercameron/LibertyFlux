//! The 32-bit differential runtime: checker macros, the shared manager
//! global and the callee stub registry.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). The call-free proof set uses `export!` only (its virtual
//! calls go through fake tables the cases plant); the factory slots
//! read the shared manager global (the route slot also reads two float
//! globals) and call callee slots, which each test binary registers
//! before running (one binary per slot shape, so the registry cells are
//! never shared between tests). Test-support code: the lifted crate
//! itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw pointers through scripted addresses are inherent
// here. Every access stays inside the test image the case built.
#![allow(unsafe_code)]

use core::sync::atomic::{AtomicU32, Ordering};

/// File VA of the shared factory-manager word.
const MANAGER_VA: u32 = 0x0167E2A0;

/// File VAs of the two float words the route slot's build call carries.
const FLOAT_A_VA: u32 = 0x00ED7E70;
const FLOAT_B_VA: u32 = 0x00ED7E68;

/// The shared manager word the factory-pair slots read.
static MANAGER: AtomicU32 = AtomicU32::new(0);

/// The two float words the route slot reads for its build call.
static FLOAT_A: AtomicU32 = AtomicU32::new(0);
static FLOAT_B: AtomicU32 = AtomicU32::new(0);

/// Sets the shared manager word for the next rewrite call.
pub fn set_manager(word: u32) {
    MANAGER.store(word, Ordering::SeqCst);
}

/// Sets the route slot's first float word for the next rewrite call.
pub fn set_float_a(word: u32) {
    FLOAT_A.store(word, Ordering::SeqCst);
}

/// Sets the route slot's second float word for the next rewrite call.
pub fn set_float_b(word: u32) {
    FLOAT_B.store(word, Ordering::SeqCst);
}

/// Pointer to the global at a file VA, mirroring
/// `lf-checker-rt::global`.
///
/// # Panics
///
/// When the address is not one of the proof set's globals: any other
/// call is a case bug.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    match file_va {
        MANAGER_VA => core::ptr::addr_of!(MANAGER) as *mut T,
        FLOAT_A_VA => core::ptr::addr_of!(FLOAT_A) as *mut T,
        FLOAT_B_VA => core::ptr::addr_of!(FLOAT_B) as *mut T,
        _ => panic!(
            "unexpected global VA {file_va:#x}: the proof set reads the manager and float words only"
        ),
    }
}

/// File VA to relocated address, mirroring `lf-checker-rt`.
///
/// The build slot reads the shared manager word through this rather
/// than through [`global`]; both spellings land on the same cell.
///
/// # Panics
///
/// When the address is not the shared manager word: the proof set
/// relocates nothing else, so any other call is a case bug. On a
/// 64-bit host any call panics: rewrites run on the 32-bit target only.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    assert!(
        file_va == MANAGER_VA,
        "unexpected relocated VA {file_va:#x}: the proof set relocates the manager word only"
    );
    #[cfg(target_arch = "x86")]
    {
        core::ptr::addr_of!(MANAGER).addr() as u32
    }
    #[cfg(not(target_arch = "x86"))]
    {
        panic!("relocated() on a 64-bit host: rewrites run on the 32-bit target only");
    }
}

/// Registered stub addresses for callee slots 1 through 8 (0 until
/// the test binary registers its stubs: a call there then faults,
/// which is a case bug). One test binary per slot shape, so no binary
/// mixes two callees on one slot.
static CALLEES: [AtomicU32; 8] = [const { AtomicU32::new(0) }; 8];

/// Registers the stub address callee `id` calls land on.
///
/// # Panics
///
/// When the id is outside slots 1 through 8.
pub fn set_callee(id: u32, addr: u32) {
    assert!(
        (1..=8).contains(&id),
        "unexpected callee id {id}: the proof set uses slots 1 through 8"
    );
    CALLEES[(id - 1) as usize].store(addr, Ordering::SeqCst);
}

/// Registers the stub address callee 1 calls land on.
pub fn set_callee1(addr: u32) {
    set_callee(1, addr);
}

/// Registers the stub address callee 2 calls land on.
pub fn set_callee2(addr: u32) {
    set_callee(2, addr);
}

/// Raw stub address for callee `id`.
///
/// # Panics
///
/// When the id is outside slots 1 through 8.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    assert!(
        (1..=8).contains(&id),
        "unexpected callee id {id}: the proof set uses slots 1 through 8"
    );
    CALLEES[(id - 1) as usize].load(Ordering::SeqCst)
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
