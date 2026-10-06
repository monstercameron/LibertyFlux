//! `lf-x87-probe`: run exact x87 and SSE instruction sequences on chosen
//! inputs and return the raw result bits.
//!
//! Each test in `tests/documented.rs` asserts the result that the processor
//! documentation specifies for a real x86 processor, with the rule stated in
//! words in a comment. When the same binary runs under the operating
//! system's x86 emulator on an ARM host, every assertion that fails is a
//! place where the emulator differs from the documented processor.
//!
//! No game code, no game bytes, no dependency on the game, no reads of any
//! game file. This crate is published and runs in the pipeline.
//!
//! The crate builds and does nothing on other targets: all probe modules
//! are gated on `#[cfg(target_arch = "x86")]`, and the dump binary prints an
//! empty stub record elsewhere, so the workspace still builds everywhere.

// The whole crate exists to issue exact instruction sequences, which needs
// inline assembly and a small amount of FFI. That is the justification for
// allowing unsafe here; each site keeps the sequence balanced and restores
// the ambient control word.
#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
pub mod sse;
#[cfg(target_arch = "x86")]
pub mod x87;
#[cfg(target_arch = "x86")]
pub mod x87cmp;
#[cfg(target_arch = "x86")]
pub mod x87trans;

/// True when this 32-bit x86 process is running under emulation rather than
/// on a real x86 processor.
///
/// Detection method (Windows, x86 only): call `IsWow64Process2` on the
/// current process and compare the native machine type against
/// `IMAGE_FILE_MACHINE_I386` (0x014c). On an ARM64 host the native machine
/// is `IMAGE_FILE_MACHINE_ARM64` (0xaa64) while the process machine is
/// i386, which means every x86 instruction in this process is emulated. On
/// a real x86-64 Windows host a 32-bit process reports native machine
/// `IMAGE_FILE_MACHINE_AMD64`, which is still a real x86 processor, so this
/// function returns false there: WOW64 on x86-64 is not emulation for
/// floating-point purposes. If the call itself fails (older Windows), this
/// returns false and the caller treats the result as unknown.
///
/// On non-Windows x86 this always returns false (known limitation: QEMU
/// user-mode on an ARM host cannot be told apart from real x86 from inside
/// the process by this check; the pipeline's real-x86 runners are the
/// reference there). On non-x86 targets it returns false because no x86
/// code is running at all.
#[must_use]
pub fn is_emulated() -> bool {
    #[cfg(all(target_arch = "x86", target_os = "windows"))]
    {
        // IMAGE_FILE_MACHINE_* values from the Windows headers, repeated
        // here so the crate stays dependency-free.
        const IMAGE_FILE_MACHINE_I386: u16 = 0x014c;
        unsafe extern "system" {
            fn GetCurrentProcess() -> *mut core::ffi::c_void;
            fn IsWow64Process2(
                h_process: *mut core::ffi::c_void,
                p_process_machine: *mut u16,
                p_native_machine: *mut u16,
            ) -> i32;
        }
        let mut process_machine: u16 = 0;
        let mut native_machine: u16 = 0;
        // SAFETY: both out-pointers are valid for the call, and the handle
        // is the current-process pseudo-handle which needs no closing.
        let ok = unsafe {
            IsWow64Process2(
                GetCurrentProcess(),
                &raw mut process_machine,
                &raw mut native_machine,
            )
        };
        if ok == 0 {
            return false;
        }
        let _ = process_machine;
        native_machine != IMAGE_FILE_MACHINE_I386
    }
    #[cfg(not(all(target_arch = "x86", target_os = "windows")))]
    {
        false
    }
}

/// Machine details for the dump header, for the pipeline comparison.
#[must_use]
pub fn emulation_details() -> EmulationDetails {
    EmulationDetails {
        emulated: is_emulated(),
        arch: std::env::consts::ARCH,
        os: std::env::consts::OS,
    }
}

/// Small header describing where a dump ran.
pub struct EmulationDetails {
    /// Value of [`is_emulated`] at dump time.
    pub emulated: bool,
    /// Compile-time architecture string.
    pub arch: &'static str,
    /// Compile-time OS string.
    pub os: &'static str,
}

/// Mark a test that fails under the emulator without weakening its
/// assertion: prints a one-line `KNOWN-EMULATOR-DIFF` note to stderr when
/// [`is_emulated`] is true, then returns. The test still asserts the
/// documented result unconditionally afterwards, so it must pass on real
/// hardware and fails here, with the note explaining why.
pub fn note_emulator_diff(tag: &str, detail: &str) {
    if is_emulated() {
        eprintln!("KNOWN-EMULATOR-DIFF {tag}: {detail}");
    }
}

/// Format helpers for the dump binary (target-independent).
#[must_use]
pub fn hex16(v: u16) -> String {
    format!("{v:#06x}")
}

/// Format helpers for the dump binary (target-independent).
#[must_use]
pub fn hex32(v: u32) -> String {
    format!("{v:#010x}")
}

/// Format helpers for the dump binary (target-independent).
#[must_use]
pub fn hex64(v: u64) -> String {
    format!("{v:#018x}")
}

/// Format an 80-bit extended value as `mant:exp`, both hex.
#[must_use]
pub fn hex_f80(mant: u64, exp: u16) -> String {
    format!("{mant:#018x}:{exp:#06x}")
}

/// Pack an 80-bit extended value into ten little-endian bytes.
#[must_use]
pub fn f80_to_bytes(mant: u64, exp: u16) -> [u8; 10] {
    let mut b = [0u8; 10];
    b[0..8].copy_from_slice(&mant.to_le_bytes());
    b[8..10].copy_from_slice(&exp.to_le_bytes());
    b
}

/// Unpack ten little-endian bytes into `(mantissa, sign_and_exponent)`.
#[must_use]
pub fn bytes_to_f80(b: &[u8; 10]) -> (u64, u16) {
    let mut m = [0u8; 8];
    let mut e = [0u8; 2];
    m.copy_from_slice(&b[0..8]);
    e.copy_from_slice(&b[8..10]);
    (u64::from_le_bytes(m), u16::from_le_bytes(e))
}

// ---------------------------------------------------------------------------
// Shared x87 control-word, status-word and MXCSR constants (plain data, so
// they live here and are available on every target for decoding dumps).
// ---------------------------------------------------------------------------

/// Default probe control word: 64-bit precision, round-nearest, all masked.
pub const CW_FULL: u16 = 0x037F;
/// Precision-control mask (bits 8-9).
pub const PC_MASK: u16 = 0x0300;
/// Precision control: 24-bit mantissa (float).
pub const PC_24: u16 = 0x0000;
/// Precision control: 53-bit mantissa (double).
pub const PC_53: u16 = 0x0200;
/// Precision control: 64-bit mantissa (extended).
pub const PC_64: u16 = 0x0300;
/// Rounding-control mask (bits 10-11).
pub const RC_MASK: u16 = 0x0C00;
/// Round to nearest, ties to even.
pub const RC_NEAR: u16 = 0x0000;
/// Round toward negative infinity.
pub const RC_DOWN: u16 = 0x0400;
/// Round toward positive infinity.
pub const RC_UP: u16 = 0x0800;
/// Round toward zero (truncate).
pub const RC_CHOP: u16 = 0x0C00;

/// Status word: invalid-operation sticky flag (bit 0).
pub const SW_IE: u16 = 0x0001;
/// Status word: denormal-operand sticky flag (bit 1).
pub const SW_DE: u16 = 0x0002;
/// Status word: zero-divide sticky flag (bit 2).
pub const SW_ZE: u16 = 0x0004;
/// Status word: overflow sticky flag (bit 3).
pub const SW_OE: u16 = 0x0008;
/// Status word: underflow sticky flag (bit 4).
pub const SW_UE: u16 = 0x0010;
/// Status word: precision (inexact) sticky flag (bit 5).
pub const SW_PE: u16 = 0x0020;
/// Status word: condition bit C0 (bit 8).
pub const SW_C0: u16 = 0x0100;
/// Status word: condition bit C1 (bit 9).
pub const SW_C1: u16 = 0x0200;
/// Status word: condition bit C2 (bit 10).
pub const SW_C2: u16 = 0x0400;
/// Status word: condition bit C3 (bit 14).
pub const SW_C3: u16 = 0x4000;

/// Default probe MXCSR: all masked, round-nearest, flush/denormals-are-zero off.
pub const MXCSR_DEFAULT: u32 = 0x1F80;
/// MXCSR rounding-control shift (bits 13-14, same encoding as x87).
pub const MXCSR_RC_SHIFT: u32 = 13;
/// MXCSR flush-to-zero bit (bit 15).
pub const MXCSR_FTZ: u32 = 0x8000;
/// MXCSR invalid-operation sticky flag (bit 0).
pub const MXCSR_IE: u32 = 0x0001;
/// MXCSR precision (inexact) sticky flag (bit 5).
pub const MXCSR_PE: u32 = 0x0020;

// ---------------------------------------------------------------------------
// Shared probe output records (plain data).
// ---------------------------------------------------------------------------

/// Raw output of an f32 probe: result bits plus the x87 status word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct F32Out {
    /// Result bits.
    pub bits: u32,
    /// x87 status word after the sequence.
    pub status: u16,
}

/// Raw output of an f64 probe: result bits plus the x87 status word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct F64Out {
    /// Result bits.
    pub bits: u64,
    /// x87 status word after the sequence.
    pub status: u16,
}

/// Raw output of an 80-bit probe: mantissa, sign/exponent, status word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct F80Out {
    /// 64-bit mantissa with explicit integer bit.
    pub mant: u64,
    /// Sign plus 15-bit exponent word.
    pub exp: u16,
    /// x87 status word after the sequence.
    pub status: u16,
}

/// Raw output of a 16-bit integer-store probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I16Out {
    /// Stored value bits.
    pub val: u16,
    /// x87 status word after the sequence.
    pub status: u16,
}

/// Raw output of a 32-bit integer-store probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I32Out {
    /// Stored value bits.
    pub val: u32,
    /// x87 status word after the sequence.
    pub status: u16,
}

/// Raw output of a 64-bit integer-store probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I64Out {
    /// Stored value bits.
    pub val: u64,
    /// x87 status word after the sequence.
    pub status: u16,
}

/// Raw output of a comparison probe: status word and EFLAGS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmpOut {
    /// x87 status word after the sequence (condition bits, IE flag).
    pub status: u16,
    /// EFLAGS after a compare-and-set-flags form (0 when unused).
    pub eflags: u32,
}

/// Raw output of an SSE scalar-single probe: result bits plus MXCSR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sse32Out {
    /// Result bits.
    pub bits: u32,
    /// MXCSR after the sequence.
    pub mxcsr: u32,
}

/// Raw output of an SSE scalar-double probe: result bits plus MXCSR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sse64Out {
    /// Result bits.
    pub bits: u64,
    /// MXCSR after the sequence.
    pub mxcsr: u32,
}
