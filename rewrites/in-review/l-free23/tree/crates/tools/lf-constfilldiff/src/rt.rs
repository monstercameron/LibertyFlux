//! The 32-bit differential runtime for the constant-fill rewrites.
//!
//! Mirrors the surface the checker builds verified rewrites against (`export!`
//! and `global`). Each global the rewrites of this crate touch is one stable
//! word; the address table below lists them and nothing else.

// Test-only runtime: raw addresses through the slots are inherent here. Every
// access stays inside the table the test planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;

/// The globals the constant-fill rewrites of this crate write, in the order the
/// differential test lays them out: three per rewrite, four rewrites.
pub const SLOT_VAS: [u32; 12] = [
    0x0106_283C, 0x0106_B6DC, 0x0110_B604,
    0x012B_6270, 0x012B_6274, 0x012B_6278,
    0x0167_E2BC, 0x0167_E2C4, 0x0167_E2CC,
    0x0167_E2B8, 0x0167_E2C0, 0x0167_E2C8,
];

/// One stable word per global of `SLOT_VAS`.
static SLOTS: [AtomicU32; 12] = [
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
