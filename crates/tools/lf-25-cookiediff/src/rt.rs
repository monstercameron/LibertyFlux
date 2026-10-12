//! The 32-bit runtime for the cookie-seal batch.
//!
//! Mirrors the surface the checker builds verified rewrites against (`export!`,
//! `global`, `relocated`). The batch reads and writes one global, the seal
//! counter, which lives in a static word here. Relocation is the identity, and
//! the lifted routine is given the same constants.

// Test-only runtime: the counter is one static word, reached only through the
// accessor below, which checks the address.
#![allow(unsafe_code)]

use core::cell::UnsafeCell;

/// The image address of the seal counter.
pub const COUNTER_VA: u32 = 0x010327A0;

/// The counter word.
struct Counter(UnsafeCell<u32>);

// SAFETY: the test programs take one lock before they touch the counter.
unsafe impl Sync for Counter {}

static COUNTER: Counter = Counter(UnsafeCell::new(0));

/// Pointer to the counter, mirroring `lf-checker-rt::global`.
///
/// # Panics
///
/// When the address is not the counter. Any other global is a test bug.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    assert_eq!(
        file_va, COUNTER_VA,
        "test bug: the cookie batch touches only the counter"
    );
    COUNTER.0.get().cast::<T>()
}

/// The relocated value of an image address. Identity in this runtime.
#[must_use]
pub const fn relocated(file_va: u32) -> u32 {
    file_va
}

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`.
#[macro_export]
macro_rules! export {
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}
