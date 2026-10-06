//! The 32-bit differential runtime: checker macros, the shared globals
//! and the callee stub registry.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). The proof set reads five globals (the task-pool manager,
//! the tick counter, the duck sample threshold, the flee length
//! threshold and the spawner's manager) and stamps one class table;
//! each diff test binary registers its own callee stubs before running
//! (one binary per method, so the registry cells are never shared
//! between tests). Test-support code: the lifted crate itself stays
//! `#![forbid(unsafe_code)]`.

// Test-only runtime: raw pointers through scripted addresses are inherent
// here. Every access stays inside the test image the case built.
#![allow(unsafe_code)]

use core::sync::atomic::{AtomicU32, Ordering};

/// File VA of the shared task-pool manager word.
const MANAGER_VA: u32 = 0x0167E2A0;
/// File VA of the tick counter the duck update reads.
const TICK_VA: u32 = 0x011735B4;
/// File VA of the float threshold the duck update compares against.
const ONE_VA: u32 = 0x00FE88E8;
/// File VA of the length threshold the flee gate compares against.
const THRESH_VA: u32 = 0x00FE876C;
/// File VA of the spawner's own manager word.
const MANAGER2_VA: u32 = 0x0171FAF4;
/// File VA of the hit-response class table (stamped by its constructor).
const HIT_VTABLE_VA: u32 = 0x00ED9FD4;
/// File VA of the goto fallback table (passed to the fallback helper).
const FALLBACK_TABLE_VA: u32 = 0x00EEF598;

/// The shared manager word the clone and start slots read.
static MANAGER: AtomicU32 = AtomicU32::new(0);
/// The tick counter the duck update reads.
static TICK: AtomicU32 = AtomicU32::new(0);
/// The float threshold the duck update compares against.
static ONE: AtomicU32 = AtomicU32::new(0);
/// The length threshold the flee gate compares against.
static THRESH: AtomicU32 = AtomicU32::new(0);
/// The spawner's own manager word.
static MANAGER2: AtomicU32 = AtomicU32::new(0);

/// Sets the shared manager word for the next rewrite call.
pub fn set_manager(word: u32) {
    MANAGER.store(word, Ordering::SeqCst);
}

/// Sets the tick counter for the next rewrite call.
pub fn set_tick(word: u32) {
    TICK.store(word, Ordering::SeqCst);
}

/// Sets the float threshold (as bits) for the next rewrite call.
pub fn set_one(bits: u32) {
    ONE.store(bits, Ordering::SeqCst);
}

/// Sets the length threshold (as bits) for the next rewrite call.
pub fn set_threshold(bits: u32) {
    THRESH.store(bits, Ordering::SeqCst);
}

/// Sets the spawner's manager word for the next rewrite call.
pub fn set_manager2(word: u32) {
    MANAGER2.store(word, Ordering::SeqCst);
}

/// Pointer to the global at a file VA, mirroring
/// `lf-checker-rt::global`.
///
/// # Panics
///
/// When the address is not one of the three globals the proof set
/// reads: any other call is a case bug.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    if file_va == MANAGER_VA {
        core::ptr::addr_of!(MANAGER) as *mut T
    } else if file_va == TICK_VA {
        core::ptr::addr_of!(TICK) as *mut T
    } else if file_va == ONE_VA {
        core::ptr::addr_of!(ONE) as *mut T
    } else if file_va == THRESH_VA {
        core::ptr::addr_of!(THRESH) as *mut T
    } else if file_va == MANAGER2_VA {
        core::ptr::addr_of!(MANAGER2) as *mut T
    } else {
        panic!("unexpected global VA {file_va:#x}: the proof set reads its five globals only")
    }
}

/// File VA to relocated address, mirroring `lf-checker-rt`.
///
/// The hit-response constructor stamps its class table through this;
/// the identity mapping pins the stamp to the table's file VA, which
/// the case asserts on the blob. The flee and goto rewrites read their
/// globals through this rather than through [`global`]; those VAs land
/// on the same cells.
///
/// # Panics
///
/// When the address is none of the above: the proof set relocates
/// nothing else, so any other call is a case bug. On a 64-bit host any
/// cell call panics: rewrites run on the 32-bit target only.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    if file_va == HIT_VTABLE_VA || file_va == FALLBACK_TABLE_VA {
        return file_va;
    }
    #[cfg(target_arch = "x86")]
    {
        if file_va == MANAGER_VA {
            core::ptr::addr_of!(MANAGER).addr() as u32
        } else if file_va == TICK_VA {
            core::ptr::addr_of!(TICK).addr() as u32
        } else if file_va == ONE_VA {
            core::ptr::addr_of!(ONE).addr() as u32
        } else if file_va == THRESH_VA {
            core::ptr::addr_of!(THRESH).addr() as u32
        } else if file_va == MANAGER2_VA {
            core::ptr::addr_of!(MANAGER2).addr() as u32
        } else {
            panic!(
                "unexpected relocated VA {file_va:#x}: the proof set relocates its five globals and two tables only"
            )
        }
    }
    #[cfg(not(target_arch = "x86"))]
    {
        let _ = file_va;
        panic!("relocated() on a 64-bit host: rewrites run on the 32-bit target only");
    }
}

/// Registered stub addresses for callee slots 1 to 8 (0 until the test
/// binary registers its stubs: a call there then faults, which is a case
/// bug). One test binary per method, so no binary mixes two callees on
/// one slot.
static CALLEE_1: AtomicU32 = AtomicU32::new(0);
static CALLEE_2: AtomicU32 = AtomicU32::new(0);
static CALLEE_3: AtomicU32 = AtomicU32::new(0);
static CALLEE_4: AtomicU32 = AtomicU32::new(0);
static CALLEE_5: AtomicU32 = AtomicU32::new(0);
static CALLEE_6: AtomicU32 = AtomicU32::new(0);
static CALLEE_7: AtomicU32 = AtomicU32::new(0);
static CALLEE_8: AtomicU32 = AtomicU32::new(0);

/// Registers the stub address calls to callee `id` land on.
///
/// # Panics
///
/// When the id is not 1 to 8: the proof set uses those slots only.
pub fn set_callee(id: u32, addr: u32) {
    match id {
        1 => CALLEE_1.store(addr, Ordering::SeqCst),
        2 => CALLEE_2.store(addr, Ordering::SeqCst),
        3 => CALLEE_3.store(addr, Ordering::SeqCst),
        4 => CALLEE_4.store(addr, Ordering::SeqCst),
        5 => CALLEE_5.store(addr, Ordering::SeqCst),
        6 => CALLEE_6.store(addr, Ordering::SeqCst),
        7 => CALLEE_7.store(addr, Ordering::SeqCst),
        8 => CALLEE_8.store(addr, Ordering::SeqCst),
        _ => panic!("unexpected callee id {id}: the proof set uses slots 1 to 8 only"),
    }
}

/// Raw stub address for callee `id`.
///
/// # Panics
///
/// When the id is not 1 to 8: the proof set uses those slots only.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    match id {
        1 => CALLEE_1.load(Ordering::SeqCst),
        2 => CALLEE_2.load(Ordering::SeqCst),
        3 => CALLEE_3.load(Ordering::SeqCst),
        4 => CALLEE_4.load(Ordering::SeqCst),
        5 => CALLEE_5.load(Ordering::SeqCst),
        6 => CALLEE_6.load(Ordering::SeqCst),
        7 => CALLEE_7.load(Ordering::SeqCst),
        8 => CALLEE_8.load(Ordering::SeqCst),
        _ => panic!("unexpected callee id {id}: the proof set uses slots 1 to 8 only"),
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
