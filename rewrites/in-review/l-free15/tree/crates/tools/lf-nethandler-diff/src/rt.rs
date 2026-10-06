//! The 32-bit differential runtime: checker macros, globals and relocated addresses.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, `global`, `relocated`). The proof set reads and writes
//! fourteen globals: the two shared handler slots and the twelve unit
//! save cells. Each global is one atomic slot; the atomics give the slots
//! stable addresses, and the differential tests hold one lock across each
//! whole test, so the rewrite's plain reads and writes through them never
//! race. The twelve replacement-handler addresses are planted per test.
//! Test-support code: the lifted crate itself stays free of `unsafe`.

// Test-only runtime: raw addresses through scripted globals are inherent
// here. Every access stays inside the test memory the case planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

/// The lone instance's shared handler slot.
static SLOT_A: AtomicU32 = AtomicU32::new(0);
/// The eleven instances' shared handler slot.
static SLOT_B: AtomicU32 = AtomicU32::new(0);
/// The twelve unit save cells, in instance order (lone first).
static SAVE_SLOTS: [AtomicU32; 12] = [
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

/// Address of the global slot for a file VA.
///
/// # Panics
///
/// When the address is not one of the fourteen globals the proof set
/// reads: a case bug, never a guess.
fn slot_for(file_va: u32) -> *mut u32 {
    match file_va {
        0x017A_CD24 => SLOT_A.as_ptr(),
        0x017A_D1B8 => SLOT_B.as_ptr(),
        0x0110_EA1C => SAVE_SLOTS[0].as_ptr(),
        0x0110_E9E4 => SAVE_SLOTS[1].as_ptr(),
        0x0110_E9D4 => SAVE_SLOTS[2].as_ptr(),
        0x0110_E9C4 => SAVE_SLOTS[3].as_ptr(),
        0x0110_E9A4 => SAVE_SLOTS[4].as_ptr(),
        0x0110_E974 => SAVE_SLOTS[5].as_ptr(),
        0x0110_EA2C => SAVE_SLOTS[6].as_ptr(),
        0x0110_E9F4 => SAVE_SLOTS[7].as_ptr(),
        0x0110_EA3C => SAVE_SLOTS[8].as_ptr(),
        0x0110_EA14 => SAVE_SLOTS[9].as_ptr(),
        0x0110_EA4C => SAVE_SLOTS[10].as_ptr(),
        0x0110_E9B4 => SAVE_SLOTS[11].as_ptr(),
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

/// The relocated addresses, planted per test: the twelve replacement
/// handlers, in instance order (lone first).
static RELOC_SLOTS: [AtomicU32; 12] = [
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

/// The replacement slot for a file VA.
///
/// # Panics
///
/// When the address is not one of the twelve handler VAs the proof set
/// relocates: a case bug, never a guess.
fn reloc_slot(file_va: u32) -> &'static AtomicU32 {
    match file_va {
        0x0110_EA18 => &RELOC_SLOTS[0],
        0x0110_E9D8 => &RELOC_SLOTS[1],
        0x0110_E9C8 => &RELOC_SLOTS[2],
        0x0110_E9B8 => &RELOC_SLOTS[3],
        0x0110_E998 => &RELOC_SLOTS[4],
        0x0110_E968 => &RELOC_SLOTS[5],
        0x0110_EA20 => &RELOC_SLOTS[6],
        0x0110_E9E8 => &RELOC_SLOTS[7],
        0x0110_EA30 => &RELOC_SLOTS[8],
        0x0110_EA08 => &RELOC_SLOTS[9],
        0x0110_EA40 => &RELOC_SLOTS[10],
        0x0110_E9A8 => &RELOC_SLOTS[11],
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// Plants the relocated address of a handler VA.
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

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export` (the proof set only needs cdecl).
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (cdecl).
        #[unsafe(no_mangle)]
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
}
