//! The 32-bit differential runtime for the task-id flag-bit rewrites.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!` for the stdcall convention and `relocated`). The rewrites read a
//! pointer table, one stable word per table entry; `relocated` returns the
//! address of that word, and the test plants in it the address of the object
//! the entry names.

// Test-only runtime: raw addresses through the slots are inherent here. Every
// access stays inside the table the test planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;

/// The pointer-table entries the rewrites read: the ids minus two to five,
/// four bytes apart, starting at the table's first word minus eight.
pub const SLOT_VAS: [u32; 8] = [
    0x0129_5CD0,
    0x0129_5CD4,
    0x0129_5CD8,
    0x0129_5CDC,
    0x0129_5CE0,
    0x0129_5CE4,
    0x0129_5CE8,
    0x0129_5CEC,
];

/// One stable word per entry of `SLOT_VAS`.
static SLOTS: [AtomicU32; 8] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

/// The slot behind a file address. An unlisted address is a test bug.
fn slot_for(file_va: u32) -> *mut u32 {
    match SLOT_VAS.iter().position(|&va| va == file_va) {
        Some(index) => SLOTS[index].as_ptr(),
        None => panic!("unexpected table VA {file_va:#x}"),
    }
}

/// The address of the word behind a file address, mirroring
/// `lf-checker-rt::relocated`.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    slot_for(file_va) as usize as u32
}

/// Plants `pointer` in the table word at `file_va`.
pub fn plant(file_va: u32, pointer: u32) {
    unsafe {
        (slot_for(file_va)).write(pointer);
    }
}

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`, for the stdcall convention.
#[macro_export]
macro_rules! export {
    (stdcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (stdcall).
        #[unsafe(no_mangle)]
        pub extern "stdcall" fn $name($($arg : $ty),*) -> $ret $body
    };
}
