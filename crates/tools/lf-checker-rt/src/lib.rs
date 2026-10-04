//! `lf-checker-rt`: tiny runtime for checker rewrites (zero dependencies).
//!
//! Usage: rewrite crates declare each export with [`export!`] in the
//! original's calling convention, derive every address from
//! [`relocated`]/[`global`] (never hard-code a mapped address), and call
//! intercepted callees through the `callee_*` macros. The worker patches
//! the data exports below after loading the rewrite DLL: the relocated
//! image base, the callee stub table, and the XMM/TLS/x87 mirrors.
//!
//! Rewrites are plain `#[unsafe(no_mangle)] pub extern "<conv>" fn` items. The
//! `export!` macro keeps the declaration uniform; the `callee_*` macros
//! call intercepted callees through the worker-owned stub table.

// The worker patches the statics below through raw pointers and every
// accessor dereferences worker-owned memory, so unsafe is inherent here.
#![allow(unsafe_code)]
// Integrated lane code, proven by the checker's regression suite; pedantic
// style lints stay off here while correctness lints (clippy::all) apply.
#![allow(clippy::pedantic)]
// The crate is no_std for rewrite DLLs; host unit tests use std.
#![cfg_attr(not(test), no_std)]

/// Relocated base of the mapped original image, patched by the worker.
/// Reads 0 until the worker loads this DLL.
#[unsafe(no_mangle)]
pub static mut CHECKER_XBASE: u32 = 0;

/// Pointer to the worker's callee stub table (256 stub addresses, 0 for
/// undeclared ids), patched by the worker.
#[unsafe(no_mangle)]
pub static mut CHECKER_CTABLE: *const u32 = core::ptr::null();

/// Pointer to the worker's mirror of the trial's scripted XMM entry values
/// (32 words: xmm0-7 low-to-high), patched by the worker. A Rust rewrite
/// cannot observe incoming vector registers any other way; the original
/// reads the same values from its registers, so equality of the values is
/// still verified, only the transport differs.
#[unsafe(no_mangle)]
pub static mut CHECKER_XMM: *const u32 = core::ptr::null();

/// One word of the trial's scripted XMM entry state: register `reg` (0-7),
/// word `i` (0-3, low to high).
#[inline(always)]
#[must_use]
pub fn xmm_word(reg: usize, i: usize) -> u32 {
    // SAFETY: the worker sets CHECKER_XMM to its 32-word mirror before any
    // trial runs; reg < 8 and i < 4 are the caller's contract.
    unsafe {
        core::ptr::addr_of!(CHECKER_XMM)
            .read()
            .add(reg * 4 + i)
            .read()
    }
}

/// Pointer to the worker's mirror of the trial's fabricated TLS slot values
/// (256 words), patched by the worker. The original reads the same values
/// through FS:[0x2c]; the rewrite reads them here.
#[unsafe(no_mangle)]
pub static mut CHECKER_TLS: *const u32 = core::ptr::null();

/// The trial's fabricated value of TLS slot `slot` (0-63).
#[inline(always)]
#[must_use]
pub fn tls_slot(slot: usize) -> u32 {
    // SAFETY: the worker sets CHECKER_TLS to its 256-word mirror before any
    // trial runs; slot < 64 is the caller's contract.
    unsafe { core::ptr::addr_of!(CHECKER_TLS).read().add(slot).read() }
}

/// Pointer to the worker's mirror of the trial's x87 entry values, patched
/// by the worker: word 0 holds the number of declared entry values, and
/// the 80-bit value of ST(i) sits at byte `16 + 16 * i` (10 bytes,
/// little-endian significand then sign and exponent). The original
/// receives the same values on its FPU stack; a Rust rewrite cannot read
/// incoming x87 registers, so it reads them here, and the worker starts the
/// rewrite with an empty FPU stack (the rewrite is treated as having
/// consumed every entry value; the x87 state check verifies the original
/// did the same).
#[unsafe(no_mangle)]
pub static mut CHECKER_X87: *const u32 = core::ptr::null();

/// Byte offset of ST(0) in the x87 mirror.
const X87_MIRROR_ST0: usize = 16;
/// Byte stride between mirror slots.
const X87_MIRROR_STRIDE: usize = 16;

/// Number of x87 entry values the contract declared for this trial (0-8).
#[inline(always)]
#[must_use]
pub fn x87_count() -> usize {
    // SAFETY: the worker sets CHECKER_X87 to its mirror before any trial
    // runs of a contract that declares x87 entry values.
    unsafe { core::ptr::addr_of!(CHECKER_X87).read().read() as usize }
}

/// The exact 80-bit entry value of ST(`i`) (`i` < 8): the significand
/// (explicit integer bit at bit 63) and the sign and biased exponent.
#[inline(always)]
#[must_use]
pub fn x87_raw(i: usize) -> (u64, u16) {
    // SAFETY: as for x87_count; i < 8 is the caller's contract, and the
    // mirror holds 8 slots.
    unsafe {
        let p = (core::ptr::addr_of!(CHECKER_X87).read() as *const u8)
            .add(X87_MIRROR_ST0 + i * X87_MIRROR_STRIDE);
        (
            (p as *const u64).read_unaligned(),
            (p.add(8) as *const u16).read_unaligned(),
        )
    }
}

/// ST(`i`) as the original would store it with `fst qword` under the
/// round-to-nearest-even control word the trampoline sets (see
/// [`f80_to_f64_bits`]). Exact for entry values declared as `f64` or `f32`.
#[inline(always)]
#[must_use]
pub fn x87_f64(i: usize) -> f64 {
    let (man, sexp) = x87_raw(i);
    f64::from_bits(f80_to_f64_bits(man, sexp))
}

/// ST(`i`) as the original would store it with `fst dword` (see
/// [`f80_to_f32_bits`]). Exact for entry values declared as `f32`.
#[inline(always)]
#[must_use]
pub fn x87_f32(i: usize) -> f32 {
    let (man, sexp) = x87_raw(i);
    f32::from_bits(f80_to_f32_bits(man, sexp))
}

/// Round an 80-bit extended value to an IEEE binary format with
/// `frac_bits` fraction bits and `exp_bits` exponent bits, exactly as an
/// x87 store does with every exception masked and rounding to nearest even:
/// overflow gives infinity, tiny results are denormalised and rounded, a
/// NaN keeps the top fraction bits and is quieted, and the encodings the
/// FPU rejects as invalid operands (pseudo-infinity, pseudo-NaN, unnormal)
/// give the negative quiet "indefinite" NaN. Zero and pseudo-denormal
/// inputs are handled as the FPU does (a zero exponent counts as 1).
fn f80_round(man: u64, sexp: u16, frac_bits: u32, exp_bits: u32) -> u64 {
    let sign = u64::from(sexp >> 15) << (frac_bits + exp_bits);
    let exp_ones: u64 = (1 << exp_bits) - 1;
    let quiet = 1u64 << (frac_bits - 1);
    let frac_mask = (1u64 << frac_bits) - 1;
    let indefinite = (1u64 << (frac_bits + exp_bits)) | (exp_ones << frac_bits) | quiet;
    let e = i32::from(sexp & 0x7FFF);
    let integer_bit = man >> 63 == 1;
    if e == 0x7FFF {
        if !integer_bit {
            return indefinite;
        }
        let fraction = man << 1;
        if fraction == 0 {
            return sign | (exp_ones << frac_bits);
        }
        return sign | (exp_ones << frac_bits) | (fraction >> (64 - frac_bits)) | quiet;
    }
    if man == 0 {
        return sign;
    }
    if e != 0 && !integer_bit {
        return indefinite;
    }
    // value = man * 2^(eff - 16383 - 63); normalise so bit 63 is set.
    let eff = if e == 0 { 1 } else { e };
    let lz = man.leading_zeros();
    let m = u128::from(man << lz);
    let bias = (1i32 << (exp_bits - 1)) - 1;
    // Both terms are small (|eff| < 2^15, lz < 64), so the casts are exact.
    let mut biased = eff - 16383 - lz as i32 + bias;
    // Low bits of m to drop: 63 - frac_bits for a normal result, more for
    // a denormal one (the result then is the denormal fraction itself).
    let drop = if biased >= 1 {
        63 - frac_bits
    } else {
        (63 - frac_bits as i32 + 1 - biased) as u32
    };
    let (mut q, rem, half) = if drop >= 128 {
        (0u128, m, u128::MAX)
    } else {
        (m >> drop, m & ((1u128 << drop) - 1), 1u128 << (drop - 1))
    };
    if rem > half || (rem == half && q & 1 == 1) {
        q += 1;
    }
    // q < 2^(frac_bits + 2), so it fits in u64.
    let mut q = q as u64;
    if biased < 1 {
        // Denormal (or rounded up to the smallest normal, which the
        // carry into the exponent field encodes by itself).
        return sign | q;
    }
    if q >> (frac_bits + 1) != 0 {
        q >>= 1;
        biased += 1;
    }
    if biased as u64 >= exp_ones {
        return sign | (exp_ones << frac_bits);
    }
    sign | ((biased as u64) << frac_bits) | (q & frac_mask)
}

/// Bits of the `f64` an x87 `fst qword` stores for the 80-bit value
/// (`man`, `sexp`), with the trampoline's control word (all exceptions
/// masked, round to nearest even).
#[must_use]
pub fn f80_to_f64_bits(man: u64, sexp: u16) -> u64 {
    f80_round(man, sexp, 52, 11)
}

/// Bits of the `f32` an x87 `fst dword` stores for the 80-bit value
/// (`man`, `sexp`), with the trampoline's control word.
#[must_use]
pub fn f80_to_f32_bits(man: u64, sexp: u16) -> u32 {
    // An f32 result occupies the low 32 bits.
    f80_round(man, sexp, 23, 8) as u32
}

/// Relocated base of the original image.
#[inline(always)]
#[must_use]
pub fn xbase() -> u32 {
    // SAFETY: the worker patches CHECKER_XBASE at load; it is read-only after.
    unsafe { core::ptr::addr_of!(CHECKER_XBASE).read() }
}

/// Convert a file VA (as seen in disassembly, image base 0x400000) to the
/// relocated address in the worker's mapping.
#[inline(always)]
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    file_va.wrapping_sub(0x400000).wrapping_add(xbase())
}

/// Pointer to a global at a file VA.
#[inline(always)]
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    relocated(file_va) as *mut T
}

/// Raw stub address for callee `id` (0 when undeclared: calling it faults,
/// which the worker reports as a rewrite failure, never silently).
#[inline(always)]
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    // SAFETY: the worker sets CHECKER_CTABLE to its 256-entry stub table
    // before any trial runs; ids below 256 are the caller's contract.
    unsafe {
        let table = core::ptr::addr_of!(CHECKER_CTABLE).read();
        if table.is_null() {
            return 0;
        }
        table.add(id as usize).read()
    }
}

/// Declare a rewrite export with the original's calling convention.
///
/// ```ignore
/// lf_checker_rt::export!(cdecl, my_rewrite(a: u32, b: u32) -> u32 {
///     a.wrapping_add(b)
/// });
/// ```
/// Conventions: `cdecl`, `stdcall`, `thiscall`, `fastcall`. The first
/// parameter of a `thiscall`/`fastcall` is passed in ECX (and the second of a
/// `fastcall` in EDX), matching 32-bit MSVC.
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the checker worker by name (cdecl).
        #[unsafe(no_mangle)]
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
    (stdcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the checker worker by name (stdcall).
        #[unsafe(no_mangle)]
        pub extern "stdcall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the checker worker by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (fastcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the checker worker by name (fastcall).
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

/// Call intercepted callee `id` with the thiscall convention: the first
/// argument is passed in ECX, the rest on the stack, callee cleans up.
/// (Folded from pilot lanes q-07/q-08.)
#[macro_export]
macro_rules! callee_thiscall {
    ($id:expr, $ret:ty, $this_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "thiscall" fn(u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($this_arg $(, $arg )*)
    }};
}

/// Call intercepted callee `id` with the fastcall convention: the first two
/// arguments are passed in ECX and EDX, the rest on the stack, callee cleans.
/// (Folded from the pilot lanes; proven by the checker's tail-thunk proofs.)
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Exact 80-bit encoding of an f64 (what `fld qword` loads; NaNs are
    /// quieted as the load does).
    fn f64_to_f80(bits: u64) -> (u64, u16) {
        let sign = ((bits >> 63) as u16) << 15;
        let e = ((bits >> 52) & 0x7FF) as i32;
        let f = bits & ((1 << 52) - 1);
        if e == 0x7FF {
            let q = if f == 0 { 0 } else { 1 << 62 };
            return ((1 << 63) | (f << 11) | q, sign | 0x7FFF);
        }
        if e == 0 {
            if f == 0 {
                return (0, sign);
            }
            let msb = 63 - f.leading_zeros() as i32;
            return (f << (63 - msb), sign | (msb - 1074 + 16383) as u16);
        }
        ((1 << 63) | (f << 11), sign | (e - 1023 + 16383) as u16)
    }

    /// A small deterministic generator for property tests.
    fn xorshift(s: &mut u64) -> u64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }

    #[test]
    fn known_values() {
        assert_eq!(f80_to_f64_bits(1 << 63, 0x3FFF), 1.0f64.to_bits());
        assert_eq!(f80_to_f32_bits(1 << 63, 0xBFFF), (-1.0f32).to_bits());
        assert_eq!(f80_to_f64_bits(0, 0x8000), (-0.0f64).to_bits());
        assert_eq!(f80_to_f64_bits(1 << 63, 0x7FFF), f64::INFINITY.to_bits());
        // The 80-bit pi rounds to the f64 and f32 nearest pi.
        let (pm, pe) = (0xC90F_DAA2_2168_C235, 0x4000);
        assert_eq!(f80_to_f64_bits(pm, pe), core::f64::consts::PI.to_bits());
        assert_eq!(f80_to_f32_bits(pm, pe), core::f32::consts::PI.to_bits());
        // Invalid encodings store the indefinite NaN.
        assert_eq!(
            f80_to_f64_bits(0x4000_0000_0000_0000, 0x7FFF),
            0xFFF8_0000_0000_0000
        );
        assert_eq!(f80_to_f32_bits(0x4000_0000_0000_0000, 0x3FFF), 0xFFC0_0000);
        // A signalling NaN is quieted and keeps its top payload bits.
        assert_eq!(f80_to_f32_bits(0xA000_0000_0000_0000, 0x7FFF), 0x7FE0_0000);
        assert_eq!(
            f80_to_f64_bits(0xC000_0000_0000_0800, 0xFFFF),
            0xFFF8_0000_0000_0001
        );
    }

    #[test]
    fn ties_round_to_even() {
        // 1 + 2^-53 is exactly halfway between two f64: rounds down to even.
        assert_eq!(
            f80_to_f64_bits((1 << 63) | (1 << 10), 0x3FFF),
            1.0f64.to_bits()
        );
        // 1 + 3 * 2^-53 is halfway with an odd lower neighbour: rounds up.
        assert_eq!(
            f80_to_f64_bits((1 << 63) | (3 << 10), 0x3FFF),
            1.0f64.to_bits() + 2
        );
        // Just above halfway rounds up.
        assert_eq!(
            f80_to_f64_bits((1 << 63) | (1 << 10) | 1, 0x3FFF),
            1.0f64.to_bits() + 1
        );
        // Rounding carries into the exponent, and past the largest finite
        // value into infinity.
        assert_eq!(f80_to_f32_bits(u64::MAX, 0x3FFF), 2.0f32.to_bits());
        assert_eq!(
            f80_to_f32_bits(u64::MAX, 0x3FFF + 127),
            f32::INFINITY.to_bits()
        );
    }

    #[test]
    fn f64_round_trip_and_f32_matches_ieee_narrowing() {
        let mut s = 0x9E37_79B9_7F4A_7C15u64;
        for i in 0..200_000u64 {
            let r = xorshift(&mut s);
            // Every exponent band, denormals and the extremes included.
            let bits = match i % 4 {
                0 => r,
                1 => r & 0x800F_FFFF_FFFF_FFFF, // denormals and zeros
                2 => (r & 0x800F_FFFF_FFFF_FFFF) | ((0x380 + (r >> 52) % 0x100) << 52), // near the f32 range edges
                _ => r & 0xFFF0_0000_0000_000F, // short significands, exact ties
            };
            let x = f64::from_bits(bits);
            if x.is_nan() {
                continue;
            }
            let (m, e) = f64_to_f80(bits);
            assert_eq!(f80_to_f64_bits(m, e), bits, "{bits:#x}");
            assert_eq!(f80_to_f32_bits(m, e), (x as f32).to_bits(), "{bits:#x}");
        }
    }

    #[test]
    fn extended_values_round_once() {
        // An extended value strictly between two f32 neighbours rounds to
        // the nearer one, and denormal f32 results round correctly.
        let one_third = (0xAAAA_AAAA_AAAA_AAAB, 0x3FFD);
        assert_eq!(
            f80_to_f32_bits(one_third.0, one_third.1),
            (1.0f32 / 3.0).to_bits()
        );
        assert_eq!(
            f80_to_f64_bits(one_third.0, one_third.1),
            (1.0f64 / 3.0).to_bits()
        );
        // 2^-149 * 1.5 rounds to 2^-148 (tie, odd neighbour), 2^-149 * 0.5
        // rounds to zero (tie, even neighbour).
        assert_eq!(f80_to_f32_bits(0xC000_0000_0000_0000, 16383 - 149), 2);
        assert_eq!(f80_to_f32_bits(1 << 63, 16383 - 150), 0);
        // Far below the smallest denormal: zero with the sign kept.
        assert_eq!(f80_to_f32_bits(1 << 63, 0x8000 | 1), 0x8000_0000);
    }
}
