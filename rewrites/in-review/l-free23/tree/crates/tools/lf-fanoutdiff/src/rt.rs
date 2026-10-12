//! The 32-bit differential runtime for the word fan-out rewrites.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!` and `global`). Each global the rewrites of this crate touch is
//! one stable word; the address table below lists them and nothing else.

// Test-only runtime: raw addresses through the slots are inherent here. Every
// access stays inside the table the test planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;

/// The globals the fan-out rewrites of this crate read or write, in the order
/// the differential test lays them out: three sources, then eighteen targets.
pub const SLOT_VAS: [u32; 28] = [
    0x01BB_3900, 0x01BB_3904, 0x01BB_3908,
    0x0111_0CD8, 0x01B4_A9A0, 0x01B4_A9A4, 0x01B4_A9A8,
    0x017AD148, 0x017AD14C, 0x017AD150,
    0x01B4B304, 0x01B4B308, 0x01B4B30C,
    0x01B4B2B0, 0x01B4B2B8, 0x01B4B2BC,
    0x01B4B310, 0x01B4B314, 0x01B4B31C,
    0x018D2190, 0x018D2194, 0x018D2198,
    0x018D21A0, 0x018D21A4, 0x018D21A8,
    0x018D2180, 0x018D2184, 0x018D2188,
];

/// One stable word per global of `SLOT_VAS`.
static SLOTS: [AtomicU32; 28] = [
    AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
    AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
    AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
    AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
    AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
    AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
    AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
];

/// The slot behind a file address. An unlisted address is a test bug.
fn slot_for(file_va: u32) -> *mut u32 {
    match SLOT_VAS.iter().position(|&va| va == file_va) {
        Some(index) => SLOTS[index].as_ptr(),
        None => panic!("unexpected global VA {file_va:#x}"),
    }
}

/// Pointer to the global at a file VA, mirroring `lf-checker-rt::global`.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    slot_for(file_va) as *mut T
}

/// Plants `bits` at the global `file_va`.
pub fn plant(file_va: u32, bits: u32) {
    unsafe {
        global::<u32>(file_va).write(bits);
    }
}

/// Reads back the word at `file_va` through a fresh raw pointer.
#[must_use]
pub fn read_back(file_va: u32) -> u32 {
    unsafe { (global::<u32>(file_va) as *const u32).read() }
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
}
