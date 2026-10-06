//! The 32-bit differential runtime: checker macros, globals and callee stubs.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `global`,
//! `relocated`). The proof set reads and writes eleven globals: the pool
//! context word, the table scale word, the release's manager and
//! auxiliary words, and the seven cursor-step record words (each holding
//! the address of its four-word record). Each global
//! is one atomic slot; the atomics give the slots stable addresses, and
//! the differential tests hold one lock across each whole test, so the
//! rewrite's plain reads and writes through them never race.
//! Test-support code: the lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses through scripted globals are inherent
// here. Every access stays inside the test memory the case planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

/// The pool context word (one address).
static CTX_SLOT: AtomicU32 = AtomicU32::new(0);
/// The indexed store's table scale word.
static SCALE_SLOT: AtomicU32 = AtomicU32::new(0);
/// The release's pool manager word (one address).
static MGR_SLOT: AtomicU32 = AtomicU32::new(0);
/// The release's auxiliary word.
static AUX_SLOT: AtomicU32 = AtomicU32::new(0);
/// The seven cursor-step record words (one record address each).
static REC_SLOTS: [AtomicU32; 7] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];
/// The revocation table's header words (second lane: two words, four bytes
/// as one word slot, one word).
static R417_A: AtomicU32 = AtomicU32::new(0);
static R417_B: AtomicU32 = AtomicU32::new(0);
static R417_C: AtomicU32 = AtomicU32::new(0);
static R417_D: AtomicU32 = AtomicU32::new(0);
/// The bare bump pool's count and base globals.
static BC_COUNT: AtomicU32 = AtomicU32::new(0);
static BC_BASE: AtomicU32 = AtomicU32::new(0);
/// The two published-object global slots.
static OBJ_SLOTS: [AtomicU32; 2] = [AtomicU32::new(0), AtomicU32::new(0)];

/// Address of the global slot for a file VA.
///
/// # Panics
///
/// When the address is not one of the eleven globals the proof set reads:
/// a case bug, never a guess.
fn slot_for(file_va: u32) -> *mut u32 {
    match file_va {
        0x0117_64C0 => CTX_SLOT.as_ptr(),
        0x0103_2F58 => SCALE_SLOT.as_ptr(),
        0x016D_D5D0 => MGR_SLOT.as_ptr(),
        0x0104_96E8 => AUX_SLOT.as_ptr(),
        0x016F_7D60 => REC_SLOTS[0].as_ptr(),
        0x012B_D0E8 => REC_SLOTS[1].as_ptr(),
        0x0166_D9EC => REC_SLOTS[2].as_ptr(),
        0x018B_6F10 => REC_SLOTS[3].as_ptr(),
        0x0163_2C60 => REC_SLOTS[4].as_ptr(),
        0x018B_6F1C => REC_SLOTS[5].as_ptr(),
        0x012E_22A4 => REC_SLOTS[6].as_ptr(),
        0x0167_CA10 => R417_A.as_ptr(),
        0x0167_CA14 => R417_B.as_ptr(),
        // The four header bytes share one word slot; byte accesses cast
        // the offset pointer back to a byte pointer.
        0x0167_CA18 => R417_C.as_ptr(),
        0x0167_CA19 => (R417_C.as_ptr() as *mut u8).wrapping_add(1) as *mut u32,
        0x0167_CA1A => (R417_C.as_ptr() as *mut u8).wrapping_add(2) as *mut u32,
        0x0167_CA1B => (R417_C.as_ptr() as *mut u8).wrapping_add(3) as *mut u32,
        0x0167_CA1C => R417_D.as_ptr(),
        0x0103_ADBC => BC_COUNT.as_ptr(),
        0x0103_ADC0 => BC_BASE.as_ptr(),
        0x012B_4160 => OBJ_SLOTS[0].as_ptr(),
        0x012B_4164 => OBJ_SLOTS[1].as_ptr(),
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

/// The relocated addresses, planted per test: thirteen pool-vector
/// stamps, then the second lane's fifteen (two entry-table pointers,
/// three revocation-table pointers, the handle-table pointer, six wide
/// stamps, two object vtables).
static RELOC_SLOTS: [AtomicU32; 27] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

/// The stamp slot for a file VA.
///
/// # Panics
///
/// When the address is not one of the thirteen stamp VAs the proof set
/// relocates: a case bug, never a guess.
fn reloc_slot(file_va: u32) -> &'static AtomicU32 {
    match file_va {
        0x00E9_791C => &RELOC_SLOTS[0],
        0x00E9_769C => &RELOC_SLOTS[1],
        0x00E9_771C => &RELOC_SLOTS[2],
        0x00E9_759C => &RELOC_SLOTS[3],
        0x00E9_789C => &RELOC_SLOTS[4],
        0x00E9_7A1C => &RELOC_SLOTS[5],
        0x00E9_751C => &RELOC_SLOTS[6],
        0x00E9_779C => &RELOC_SLOTS[7],
        0x00E9_7A9C => &RELOC_SLOTS[8],
        0x00E9_799C => &RELOC_SLOTS[9],
        0x00E9_761C => &RELOC_SLOTS[10],
        0x00E9_7B1C => &RELOC_SLOTS[11],
        0x00E9_781C => &RELOC_SLOTS[12],
        0x0120_F2B8 => &RELOC_SLOTS[13],
        0x0120_F2C0 => &RELOC_SLOTS[14],
        0x0167_CA20 => &RELOC_SLOTS[15],
        0x0167_0D29 => &RELOC_SLOTS[16],
        0x0167_CA39 => &RELOC_SLOTS[17],
        0x0129_5CD8 => &RELOC_SLOTS[18],
        0x00E9_7B9C => &RELOC_SLOTS[19],
        0x00E9_7C5C => &RELOC_SLOTS[20],
        0x00E9_7C9C => &RELOC_SLOTS[21],
        0x00E9_7BDC => &RELOC_SLOTS[22],
        0x00E9_7D1C => &RELOC_SLOTS[23],
        0x00E9_7CDC => &RELOC_SLOTS[24],
        0x00E9_7EC0 => &RELOC_SLOTS[25],
        0x00E9_7EA0 => &RELOC_SLOTS[26],
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// Plants the relocated address of a stamp VA.
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

/// Registered stub addresses for callee ids 0..5 (0 when none: a call
/// there panics, which is a case bug). Each test plants the stubs its
/// rewrites call before running.
static CALLEES: [AtomicU32; 5] = [
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
