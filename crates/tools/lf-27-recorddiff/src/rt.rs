//! The 32-bit differential runtime: the checker's names for the export macro,
//! globals and relocation, over one window of test memory.
//!
//! The rewrites of this family clear records at fixed file addresses, and some
//! of them clear a flag byte just before the run. The runtime maps one window
//! of file addresses onto a block of test memory: a file address inside the
//! window is the matching byte of the block. Each test sets the window before
//! it calls a rewrite. Both [`relocated`] and [`global`] map through the same
//! window, so the rewrites' raw address writes and their global writes land in
//! the same bytes.

use core::sync::atomic::{AtomicU32, Ordering};

/// The first file address of the window.
static WINDOW_VA: AtomicU32 = AtomicU32::new(0);
/// The test-memory address of the window's first byte.
static WINDOW_ADDR: AtomicU32 = AtomicU32::new(0);
/// The window's length in bytes.
static WINDOW_LEN: AtomicU32 = AtomicU32::new(0);

/// Sets the window: the file address `va` is the test-memory address `addr`,
/// and `len` bytes from it are mapped.
pub fn set_window(va: u32, addr: u32, len: u32) {
    WINDOW_VA.store(va, Ordering::Relaxed);
    WINDOW_ADDR.store(addr, Ordering::Relaxed);
    WINDOW_LEN.store(len, Ordering::Relaxed);
}

/// The test-memory address of a file address in the window.
///
/// # Panics
///
/// When `file_va` lies outside the window: a rewrite touched memory the test
/// did not map, which is a test bug.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    let va = WINDOW_VA.load(Ordering::Relaxed);
    let len = WINDOW_LEN.load(Ordering::Relaxed);
    let offset = file_va.wrapping_sub(va);
    assert!(
        offset < len,
        "file address {file_va:#x} lies outside the test window"
    );
    WINDOW_ADDR.load(Ordering::Relaxed).wrapping_add(offset)
}

/// A global of the original: the window byte at its file address.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    relocated(file_va) as *mut T
}

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export` for the cdecl form these rewrites use.
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (cdecl).
        #[unsafe(no_mangle)]
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
}
