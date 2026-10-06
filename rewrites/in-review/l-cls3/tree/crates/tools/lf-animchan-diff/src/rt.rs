//! The 32-bit differential runtime: checker macros and shared constants.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`, `xmm_word`). What the samplers need is `global`, answered
//! from the measured constant values; the one rewrite that reads a
//! constant through `relocated` gets the same answer there. Two cases
//! exercise callees: each test registers its stub (the square root, or
//! the lifted segment evaluator) before running. Test-support code: the
//! lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw pointers through scripted addresses are inherent
// here. Every access stays inside the test image the case built.
#![allow(unsafe_code)]

use core::sync::atomic::{AtomicU32, Ordering};

/// The shared rounding half, measured from the executable's data.
static ROUND_HALF_BITS: u32 = 0x3F00_0000;
/// The round-to-nearest bias (2^23), measured the same way.
static ROUND_MAGIC_BITS: u32 = 0x4B00_0000;
/// One, measured the same way.
static ONE_BITS: u32 = 0x3F80_0000;
/// The snap-down threshold, measured the same way.
static SNAP_LO_BITS: u32 = 0x3A83_126F;
/// The snap-up threshold, measured the same way.
static SNAP_HI_BITS: u32 = 0x3F7F_BE77;

/// The shared constant at a file VA, if it is one of the five the proof
/// set reads.
fn const_bits(file_va: u32) -> Option<*const u32> {
    match file_va {
        0xFE86B4 => Some(&SNAP_LO_BITS),
        0xFE88DC => Some(&SNAP_HI_BITS),
        0xFE88E8 => Some(&ONE_BITS),
        0xFE8CF8 => Some(&ROUND_MAGIC_BITS),
        0xFE8830 => Some(&ROUND_HALF_BITS),
        _ => None,
    }
}

/// Pointer to the shared constant at a file VA, mirroring
/// `lf-checker-rt::global` (image base `0x400000`).
///
/// # Panics
///
/// When the address is not one of the five constants the proof set
/// reads: a case bug, never a guess.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    let word = const_bits(file_va).unwrap_or_else(|| panic!("unexpected shared constant VA {file_va:#x}"));
    word as *mut T
}

/// Relocated base, mirroring `lf-checker-rt` (the proof set never reads
/// it: every relocated address it touches is a shared constant, but the
/// name must resolve).
pub static mut CHECKER_XBASE: u32 = 0;

/// File VA to relocated address, mirroring `lf-checker-rt`. The five
/// shared constants answer from the table above; anything else follows
/// the relocated base.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    if let Some(word) = const_bits(file_va) {
        return word as usize as u32;
    }
    let xbase = unsafe { core::ptr::addr_of!(CHECKER_XBASE).read() };
    file_va.wrapping_sub(0x400000).wrapping_add(xbase)
}

/// Low word of the emulated xmm0 register: the segment evaluator reads
/// its `t` there, and each case primes it before calling.
static XMM0_LO: AtomicU32 = AtomicU32::new(0);

/// Primes the emulated xmm0 low word for the next rewrite call.
pub fn set_xmm0_lo(bits: u32) {
    XMM0_LO.store(bits, Ordering::SeqCst);
}

/// One word of an emulated vector register, mirroring
/// `lf-checker-rt::xmm_word`.
///
/// # Panics
///
/// When the case asks for anything but register 0, word 0: the proof set
/// transports a single float there, nothing else.
#[must_use]
pub fn xmm_word(reg: u32, lane: u32) -> u32 {
    assert!(reg == 0 && lane == 0, "unexpected xmm read ({reg}, {lane})");
    XMM0_LO.load(Ordering::SeqCst)
}

/// Registered stub address for callee 1 (0 when none: a call there then
/// faults, which is a case bug). Each callee case registers its own stub
/// before running; no test binary mixes two callees.
static CALLEE1: AtomicU32 = AtomicU32::new(0);

/// Registers the stub address callee 1 calls land on.
pub fn set_callee1(addr: u32) {
    CALLEE1.store(addr, Ordering::SeqCst);
}

/// Raw stub address for callee `id`: the registered stub for id 1, 0
/// otherwise.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    if id == 1 {
        CALLEE1.load(Ordering::SeqCst)
    } else {
        0
    }
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
    (stdcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (stdcall).
        #[unsafe(no_mangle)]
        pub extern "stdcall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (fastcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (fastcall).
        #[unsafe(no_mangle)]
        pub extern "fastcall" fn $name($($arg : $ty),*) -> $ret $body
    };
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

/// Call intercepted callee `id` with the stdcall convention.
#[macro_export]
macro_rules! callee_stdcall {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        let f: extern "stdcall" fn($( $crate::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($( $arg ),*)
    }};
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

/// Call intercepted callee `id` with the fastcall convention.
#[macro_export]
macro_rules! callee_fastcall {
    ($id:expr, $ret:ty, $ecx_arg:expr, $edx_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "fastcall" fn(u32, u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($ecx_arg, $edx_arg $(, $arg )*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
