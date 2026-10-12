//! The 32-bit differential runtime: the checker's global names for the words this crate touches.
//!
//! Mirrors the surface the checker builds verified rewrites against (`export!`, `global`). Each global
//! the rewrites of this crate touch is one stable word, and the address table below lists them and
//! nothing else.

// Test-only runtime: raw addresses through scripted globals are inherent here. Every access stays
// inside the test memory the case planted.
#![allow(unsafe_code)]

use core::sync::atomic::{AtomicU32, Ordering};

/// The number of words the runtime holds.
pub const WORD_COUNT: usize = 15;

/// One stable word per global the rewrites of this crate touch.
static SLOTS: [AtomicU32; 15] = [
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

/// The slot behind a file address. An unlisted address is a test bug.
fn slot_for(file_va: u32) -> *mut u32 {
    match file_va {
        0x017AD148 => SLOTS[0].as_ptr(),
        0x01B4B330 => SLOTS[1].as_ptr(),
        0x01B4B2F4 => SLOTS[2].as_ptr(),
        0x01B4B2C8 => SLOTS[3].as_ptr(),
        0x01B4B2EC => SLOTS[4].as_ptr(),
        0x01BB38FC => SLOTS[5].as_ptr(),
        0x012BD1D0 => SLOTS[6].as_ptr(),
        0x0103B934 => SLOTS[7].as_ptr(),
        0x01048AB4 => SLOTS[8].as_ptr(),
        0x0103C060 => SLOTS[9].as_ptr(),
        0x016D8B14 => SLOTS[10].as_ptr(),
        0x01050B4C => SLOTS[11].as_ptr(),
        0x0171DE54 => SLOTS[12].as_ptr(),
        0x00ED7CDC => SLOTS[13].as_ptr(),
        0x0171FAC4 => SLOTS[14].as_ptr(),
        _ => panic!("unexpected global VA {file_va:#x}"),
    }
}

/// Pointer to the global at a file VA, mirroring `lf-checker-rt::global`.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    slot_for(file_va) as *mut T
}

/// Sets the word with the given number. The test binds numbers to the instance table.
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
}
