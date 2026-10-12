//! The 32-bit differential runtime: the checker's global names for the static words this crate reads.
//!
//! Mirrors the surface the checker builds verified rewrites against (`export!`, `global`). Each global
//! the rewrites of this crate read is one stable word, and the address table below lists them and
//! nothing else.

// Test-only runtime: raw addresses through scripted globals are inherent here.
#![allow(unsafe_code)]

use core::sync::atomic::{AtomicU32, Ordering};

/// The number of words the runtime holds.
pub const WORD_COUNT: usize = 2;

/// One stable word per global the rewrites of this crate read.
static SLOTS: [AtomicU32; 2] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
];

/// The slot behind a file address. An unlisted address is a test bug.
fn slot_for(file_va: u32) -> *mut u32 {
    match file_va {
        0x011D6FC4 => SLOTS[0].as_ptr(),
        0x011D7680 => SLOTS[1].as_ptr(),
        _ => panic!("unexpected global VA {file_va:#x}"),
    }
}

/// Pointer to the global at a file VA, mirroring `lf-checker-rt::global`.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    slot_for(file_va) as *mut T
}

/// Sets the word with the given number.
pub fn set_word(number: usize, value: u32) {
    SLOTS[number].store(value, Ordering::SeqCst);
}

/// Reads the word with the given number.
#[must_use]
pub fn get_word(number: usize) -> u32 {
    SLOTS[number].load(Ordering::SeqCst)
}

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`.
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (cdecl).
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}
