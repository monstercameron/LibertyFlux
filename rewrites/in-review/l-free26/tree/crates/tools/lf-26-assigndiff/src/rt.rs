//! The 32-bit differential runtime for the indexed-slot family.
//!
//! Mirrors the surface the checker builds verified rewrites against (`export!`, `global`). The one global
//! these rewrites read is a table of words; it is a static array of atomic words, and the test writes the
//! table's contents into it before each case.

// Test-only runtime: raw addresses through scripted globals are inherent here. Every access stays inside
// the test memory the case planted.
#![allow(unsafe_code)]

use core::sync::atomic::{AtomicU32, Ordering};

/// The file address of the table the rewrites read.
pub const TABLE_BASE: u32 = 0x0103_2FAC;

/// Words in the table. Every index a case uses is below this, so the rewrite never reads past it.
pub const TABLE_LEN: usize = 64;

/// The table's words.
static TABLE: [AtomicU32; TABLE_LEN] = [const { AtomicU32::new(0) }; TABLE_LEN];

/// Pointer to the global at a file address, mirroring `lf-checker-rt::global`. Only the table exists here.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    assert_eq!(file_va, TABLE_BASE, "unexpected global VA {file_va:#x}");
    TABLE[0].as_ptr().cast::<T>()
}

/// Writes `words` into the table, word for word.
pub fn plant_table(words: &[u32; TABLE_LEN]) {
    for (slot, &word) in TABLE.iter().zip(words.iter()) {
        slot.store(word, Ordering::Relaxed);
    }
}

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`.
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (cdecl).
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
    (stdcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (stdcall).
        pub extern "stdcall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}
