//! The 32-bit differential runtime for the slot-record store rewrites.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!` for the thiscall convention and `global`). The rewrites read
//! eight selector-table words, one stable word per table entry; the address
//! table below lists them and nothing else.

// Test-only runtime: raw addresses through the slots are inherent here. Every
// access stays inside the table the test planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;

/// The selector-table globals the rewrites read: eight words, spaced eight
/// bytes apart from the table's first word.
pub const SLOT_VAS: [u32; 8] = [
    0x0115_F808, 0x0115_F810, 0x0115_F818, 0x0115_F820,
    0x0115_F828, 0x0115_F830, 0x0115_F838, 0x0115_F840,
];

/// One stable word per table global of `SLOT_VAS`.
static SLOTS: [AtomicU32; 8] = [
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
/// Mirrors `lf-checker-rt::export`, for the thiscall convention.
#[macro_export]
macro_rules! export {
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}
