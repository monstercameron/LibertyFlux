//! The 32-bit differential runtime: checker macros, callee stubs and the
//! planted image window.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `relocated` and `global`). The
//! window is one block of planted memory at a fixed file address; `global`
//! and `relocated` map file addresses inside it to real pointers, so a
//! rewrite reads and writes the table the test planted. Each test program
//! plants the callee stubs its rewrites need, because callee numbers mean
//! different things in different rewrites. Test-support code: the lifted
//! crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses are inherent here. Every call goes to a
// stub the test planted, and every read goes to the planted window.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

/// Registered stub addresses for callee ids 0..8 (0 when none: a call
/// there panics, which is a case bug).
static CALLEES: [AtomicU32; 8] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

/// Plants the stub address that callee `id` calls land on.
pub fn set_callee(id: u32, addr: u32) {
    CALLEES[id as usize].store(addr, Ordering::Relaxed);
}

/// Raw stub address for callee `id`.
///
/// # Panics
///
/// When `id` is outside the table or no stub is planted: a case bug.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    let slot = CALLEES
        .get(id as usize)
        .unwrap_or_else(|| panic!("unexpected callee id {id}"));
    let addr = slot.load(Ordering::Relaxed);
    assert!(addr != 0, "callee {id} called with no stub planted");
    addr
}

/// First file address of the planted window.
pub const WINDOW_VA: u32 = 0x0129_2400;
/// Length of the planted window in bytes. It covers the 32-record table and
/// the address one past its end that one routine returns.
pub const WINDOW_LEN: u32 = 0x5000;

/// First file address of the small auxiliary window, which holds the game
/// clock word that the refresh routine reads as a global.
pub const AUX_VA: u32 = 0x0117_35A0;
/// Length of the auxiliary window in bytes.
pub const AUX_LEN: u32 = 0x40;

/// Pointer to the first byte of the planted window in this process.
static WINDOW_PTR: AtomicU32 = AtomicU32::new(0);
/// Pointer to the first byte of the auxiliary window in this process.
static AUX_PTR: AtomicU32 = AtomicU32::new(0);
/// Extra planted addresses: file addresses outside the windows (a helper's
/// descriptor, a game global) and the process addresses planted for them.
static EXTRA: std::sync::Mutex<Vec<(u32, u32)>> = std::sync::Mutex::new(Vec::new());

/// Plants the process address of one extra file address.
pub fn set_extra(file_va: u32, ptr: u32) {
    let mut table = EXTRA.lock().unwrap_or_else(|poison| poison.into_inner());
    table.retain(|(va, _)| *va != file_va);
    table.push((file_va, ptr));
}

/// Process address of the planted window (the value of [WINDOW_VA]).
pub fn window_base() -> u32 {
    WINDOW_PTR.load(Ordering::Relaxed)
}

/// Plants the window: `ptr` is the process address of [`WINDOW_VA`].
pub fn set_window(ptr: u32) {
    WINDOW_PTR.store(ptr, Ordering::Relaxed);
}

/// Plants the auxiliary window: `ptr` is the process address of [`AUX_VA`].
pub fn set_aux(ptr: u32) {
    AUX_PTR.store(ptr, Ordering::Relaxed);
}

/// Writes one word of the auxiliary window at a file address inside it.
///
/// # Panics
///
/// When no auxiliary window is planted or the address lies outside it.
pub fn set_aux_word(file_va: u32, value: u32) {
    let offset = file_va.wrapping_sub(AUX_VA);
    let base = AUX_PTR.load(Ordering::Relaxed);
    assert!(base != 0, "no auxiliary window planted");
    assert!(
        offset + 4 <= AUX_LEN,
        "address {file_va:#x} outside the auxiliary window"
    );
    // SAFETY: the auxiliary window is planted at `base` with AUX_LEN bytes
    // of writable memory; the offset and width were checked above.
    unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) };
}

/// Relocated address of a file address inside a planted window.
///
/// # Panics
///
/// When no window is planted or the address lies outside them: a case bug.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    let offset = file_va.wrapping_sub(WINDOW_VA);
    let base = WINDOW_PTR.load(Ordering::Relaxed);
    if base != 0 && offset < WINDOW_LEN {
        return base.wrapping_add(offset);
    }
    let aux_offset = file_va.wrapping_sub(AUX_VA);
    let aux = AUX_PTR.load(Ordering::Relaxed);
    if aux != 0 && aux_offset < AUX_LEN {
        return aux.wrapping_add(aux_offset);
    }
    let found = EXTRA
        .lock()
        .unwrap_or_else(|poison| poison.into_inner())
        .iter()
        .find(|(va, _)| *va == file_va)
        .map(|(_, ptr)| *ptr);
    found.unwrap_or_else(|| panic!("file address {file_va:#x} lies outside the planted windows"))
}

/// Pointer to a global at a file address, as the checker's runtime gives it.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    relocated(file_va) as *mut T
}

/// Writes one word at a planted extra address (a game global's value).
///
/// # Panics
///
/// When the address was not planted: a case bug.
pub fn set_extra_word(file_va: u32, value: u32) {
    let ptr = relocated(file_va);
    // SAFETY: `relocated` returns a planted extra address, a writable word
    // the test allocated for this global.
    unsafe { (ptr as *mut u32).write_unaligned(value) };
}

/// Copies `bytes` into the planted window. `bytes` must be [`WINDOW_LEN`]
/// long.
///
/// # Panics
///
/// When no window is planted or the length is wrong: a case bug.
pub fn write_window(bytes: &[u8]) {
    assert_eq!(bytes.len(), WINDOW_LEN as usize, "window length");
    let base = WINDOW_PTR.load(Ordering::Relaxed);
    assert!(base != 0, "no window planted");
    // SAFETY: the window is planted at `base` with WINDOW_LEN bytes of
    // writable memory that outlives the test case; the length was checked.
    unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), base as *mut u8, bytes.len()) };
}

/// Copies the planted window out.
///
/// # Panics
///
/// When no window is planted: a case bug.
#[must_use]
pub fn read_window() -> Vec<u8> {
    let base = WINDOW_PTR.load(Ordering::Relaxed);
    assert!(base != 0, "no window planted");
    let mut out = vec![0_u8; WINDOW_LEN as usize];
    // SAFETY: the window is planted at `base` with WINDOW_LEN bytes of
    // readable memory; `out` has the same length and cannot overlap it.
    unsafe { core::ptr::copy_nonoverlapping(base as *const u8, out.as_mut_ptr(), out.len()) };
    out
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
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (cdecl).
        #[unsafe(no_mangle)]
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
}

/// Call intercepted callee `id` with the thiscall convention.
#[macro_export]
macro_rules! callee_thiscall {
    ($id:expr, $ret:ty, $this_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "thiscall" fn(u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($this_arg $(, $arg )*)
    }};
}

/// Call intercepted callee `id` with the cdecl convention.
#[macro_export]
macro_rules! callee_cdecl {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        let f: extern "cdecl" fn($( $crate::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($( $arg ),*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
