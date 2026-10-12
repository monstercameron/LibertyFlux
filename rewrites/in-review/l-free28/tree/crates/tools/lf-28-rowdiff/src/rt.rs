//! The 32-bit differential runtime for the global-word rewrites of this crate.
//!
//! Mirrors the surface the checker builds verified rewrites against (`export!`, `global` and `relocated`).
//! The rewrites address globals by absolute address. Here one region of words stands for them: the address
//! `base` the test sets is the region's first word, and every other global is a word further on.

// Test-only runtime: raw pointers into the region are inherent here. Every access is bounds-checked against
// the region before it is made.
#![allow(unsafe_code)]

use core::sync::atomic::{AtomicU32, Ordering};

/// Words in the region. Every global an instance of this crate touches lies inside it.
pub const REGION_WORDS: usize = 0x2000;

/// The region's words, as atomics so that the region has a stable address and no mutable static.
static REGION: [AtomicU32; REGION_WORDS] = [const { AtomicU32::new(0) }; REGION_WORDS];

/// The global address of the region's first word.
static REGION_BASE: AtomicU32 = AtomicU32::new(0);

/// Points the globals at a region that starts at the global address `base`.
pub fn set_region(base: u32) {
    REGION_BASE.store(base, Ordering::SeqCst);
}

/// The word of the region at a global address. An address outside the region is a test bug.
fn word_for(file_va: u32) -> &'static AtomicU32 {
    let offset = file_va.wrapping_sub(REGION_BASE.load(Ordering::SeqCst)) as usize;
    assert!(offset % 4 == 0 && offset / 4 < REGION_WORDS, "global VA {file_va:#x} lies outside the region");
    &REGION[offset / 4]
}

/// Pointer to the global at a global address, mirroring `lf-checker-rt::global`.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    word_for(file_va).as_ptr().cast::<T>()
}

/// The address of the global, as the rewrites compute with it, mirroring `lf-checker-rt::relocated`.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    word_for(file_va).as_ptr() as usize as u32
}

/// Writes every word of the region from `words`.
pub fn fill_region(words: &[u32]) {
    for (slot, value) in REGION.iter().zip(words.iter()) {
        slot.store(*value, Ordering::SeqCst);
    }
}

/// The region's words as they are now.
#[must_use]
pub fn region_words() -> Vec<u32> {
    REGION.iter().map(|slot| slot.load(Ordering::SeqCst)).collect()
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

/// Writes one word of the region at a global address.
pub fn write_word(file_va: u32, value: u32) {
    word_for(file_va).store(value, Ordering::SeqCst);
}

/// Reads one word of the region at a global address.
#[must_use]
pub fn read_word(file_va: u32) -> u32 {
    word_for(file_va).load(Ordering::SeqCst)
}
