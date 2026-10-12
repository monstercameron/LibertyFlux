//! The 32-bit differential runtime for the static-row batch.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, `global`, `relocated`). The rewrites of this crate touch one
//! block of words in the image's data; the table below is that block, as
//! words, and every access stays inside it. Relocation is the identity here:
//! the rewrites' own constants are used as they are, and the lifted method is
//! given the same constants.

// Test-only runtime: raw addresses through a scripted table are inherent here.
// Every access stays inside the table, which the global accessor checks.
#![allow(unsafe_code)]

use core::cell::UnsafeCell;

/// The image address of the first word of the table.
pub const MEM_BASE: u32 = 0x0110DF20;

/// The number of 32-bit words in the table.
pub const MEM_WORDS: usize = 2092;

/// The table: aligned to eight bytes, so a 64-bit store to a word-aligned
/// address is an ordinary store to the table.
#[repr(C, align(8))]
struct Memory(UnsafeCell<[u32; MEM_WORDS]>);

// SAFETY: the test programs take one lock before they touch the table, so no
// two threads use it at once.
unsafe impl Sync for Memory {}

static MEMORY: Memory = Memory(UnsafeCell::new([0; MEM_WORDS]));

/// Pointer to the table at an image address, mirroring `lf-checker-rt::global`.
///
/// # Panics
///
/// When the access does not lie inside the table. An access outside it is a
/// test bug.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    let size = core::mem::size_of::<T>() as u32;
    let end = MEM_BASE + 4 * MEM_WORDS as u32;
    assert!(
        file_va >= MEM_BASE && file_va.checked_add(size).is_some_and(|e| e <= end),
        "test bug: access at {file_va:#x} lies outside the table"
    );
    let offset = (file_va - MEM_BASE) as usize;
    // SAFETY: the offset and size are inside the table, checked above.
    unsafe { (MEMORY.0.get() as *mut u8).add(offset) as *mut T }
}

/// The relocated value of an image address. Identity in this runtime.
#[must_use]
pub const fn relocated(file_va: u32) -> u32 {
    file_va
}

/// Plants `bits` at the word `file_va`.
pub fn plant(file_va: u32, bits: u32) {
    // SAFETY: `global` checks the access lies inside the table.
    unsafe {
        global::<u32>(file_va).write(bits);
    }
}

/// Reads back the word at `file_va`.
#[must_use]
pub fn read_back(file_va: u32) -> u32 {
    // SAFETY: `global` checks the access lies inside the table.
    unsafe { global::<u32>(file_va).read() }
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
