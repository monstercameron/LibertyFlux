//! x87 probes part 1: control-word helpers, group 1 (loads, stores and
//! float-stack returns) and group 2 (precision control). 32-bit x86 only.
//!
//! Every probe saves the ambient control word, installs its own (always
//! with all exceptions masked, so nothing traps), clears the status flags,
//! runs one balanced float-stack sequence, reads the status word, then
//! clears the flags again and restores the ambient word. No probe leaves a
//! value on the float stack.

use core::arch::asm;

use crate::{CW_FULL, F32Out, F64Out, F80Out, PC_MASK, bytes_to_f80, f80_to_bytes};

/// Read the x87 control word.
pub(crate) fn read_cw() -> u16 {
    let mut cw: u16 = 0;
    unsafe {
        asm!(
            "fstcw word ptr [{0}]",
            in(reg) &raw mut cw,
            options(preserves_flags),
        );
    }
    cw
}

/// Load the x87 control word.
pub(crate) fn write_cw(cw: u16) {
    unsafe {
        asm!(
            "fldcw word ptr [{0}]",
            in(reg) &raw const cw,
            options(preserves_flags),
        );
    }
}

/// Read the x87 status word (no-wait form, so a pending flag never traps).
pub(crate) fn read_sw() -> u16 {
    let status: u16;
    unsafe {
        asm!(
            "fnstsw ax",
            out("ax") status,
            options(preserves_flags),
        );
    }
    status
}

/// Clear the x87 exception flags.
pub(crate) fn clear_exceptions() {
    unsafe {
        asm!("fnclex", options(preserves_flags));
    }
}

/// Run `f` under control word `cw`, restoring the ambient word afterwards.
pub(crate) fn with_cw<R>(cw: u16, f: impl FnOnce() -> R) -> R {
    let saved = read_cw();
    write_cw(cw);
    clear_exceptions();
    let out = f();
    clear_exceptions();
    write_cw(saved);
    out
}

// ---------------------------------------------------------------------------
// Group 1: loads and stores between memory and the float stack.
// ---------------------------------------------------------------------------

/// Group 1: `fld m32` then `fstp m32` at full precision, all masked.
#[must_use]
pub fn g1_f32_mem_roundtrip(input: u32) -> F32Out {
    with_cw(CW_FULL, || {
        let mut out: u32 = 0;
        unsafe {
            asm!(
                "fld dword ptr [{0}]",
                "fstp dword ptr [{1}]",
                in(reg) &raw const input,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        F32Out {
            bits: out,
            status: read_sw(),
        }
    })
}

/// Group 1: `fld m64` then `fstp m64` at full precision, all masked.
#[must_use]
pub fn g1_f64_mem_roundtrip(input: u64) -> F64Out {
    with_cw(CW_FULL, || {
        let mut out: u64 = 0;
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fstp qword ptr [{1}]",
                in(reg) &raw const input,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        F64Out {
            bits: out,
            status: read_sw(),
        }
    })
}

/// Group 1: `fld m80` then `fstp m80` at full precision, all masked.
#[must_use]
pub fn g1_f80_mem_roundtrip(mant: u64, exp: u16) -> F80Out {
    let input = f80_to_bytes(mant, exp);
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld tbyte ptr [{0}]",
                "fstp tbyte ptr [{1}]",
                in(reg) &raw const input,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        let (m, e) = bytes_to_f80(&out);
        F80Out {
            mant: m,
            exp: e,
            status: read_sw(),
        }
    })
}

/// Group 1: 32-bit value handed back on the float stack, as a 32-bit
/// function returns a float: a real `call`/`ret` pair with the value loaded
/// by the callee (`fld m32`) and stored by the caller (`fstp m32`).
#[must_use]
#[inline(never)]
pub fn g1_f32_return_roundtrip(input: u32) -> F32Out {
    with_cw(CW_FULL, || {
        let mut out: u32 = 0;
        unsafe {
            asm!(
                "call 2f",
                "fstp dword ptr [{1}]",
                "jmp 3f",
                "2:",
                "fld dword ptr [{0}]",
                "ret",
                "3:",
                in(reg) &raw const input,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        F32Out {
            bits: out,
            status: read_sw(),
        }
    })
}

/// Group 1: 64-bit value handed back on the float stack, as a 32-bit
/// function returns a double: a real `call`/`ret` pair with the value
/// loaded by the callee (`fld m64`) and stored by the caller (`fstp m64`).
#[must_use]
#[inline(never)]
pub fn g1_f64_return_roundtrip(input: u64) -> F64Out {
    with_cw(CW_FULL, || {
        let mut out: u64 = 0;
        unsafe {
            asm!(
                "call 2f",
                "fstp qword ptr [{1}]",
                "jmp 3f",
                "2:",
                "fld qword ptr [{0}]",
                "ret",
                "3:",
                in(reg) &raw const input,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        F64Out {
            bits: out,
            status: read_sw(),
        }
    })
}

// ---------------------------------------------------------------------------
// Group 2: precision control. Inputs are doubles; the result is stored as
// 80-bit extended so the rounded mantissa is visible with trailing zeros.
// `pc` holds raw precision bits (`PC_24`, `PC_53` or `PC_64`).
// ---------------------------------------------------------------------------

/// Group 2: add at the given precision.
#[must_use]
pub fn g2_fadd(pc: u16, a: u64, b: u64) -> F80Out {
    let cw = (CW_FULL & !PC_MASK) | (pc & PC_MASK);
    with_cw(cw, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "faddp",
                "fstp tbyte ptr [{2}]",
                in(reg) &raw const a,
                in(reg) &raw const b,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        let (m, e) = bytes_to_f80(&out);
        F80Out {
            mant: m,
            exp: e,
            status: read_sw(),
        }
    })
}

/// Group 2: subtract (`a - b`) at the given precision.
#[must_use]
pub fn g2_fsub(pc: u16, a: u64, b: u64) -> F80Out {
    let cw = (CW_FULL & !PC_MASK) | (pc & PC_MASK);
    with_cw(cw, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fsubp",
                "fstp tbyte ptr [{2}]",
                in(reg) &raw const a,
                in(reg) &raw const b,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        let (m, e) = bytes_to_f80(&out);
        F80Out {
            mant: m,
            exp: e,
            status: read_sw(),
        }
    })
}

/// Group 2: multiply at the given precision.
#[must_use]
pub fn g2_fmul(pc: u16, a: u64, b: u64) -> F80Out {
    let cw = (CW_FULL & !PC_MASK) | (pc & PC_MASK);
    with_cw(cw, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fmulp",
                "fstp tbyte ptr [{2}]",
                in(reg) &raw const a,
                in(reg) &raw const b,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        let (m, e) = bytes_to_f80(&out);
        F80Out {
            mant: m,
            exp: e,
            status: read_sw(),
        }
    })
}

/// Group 2: divide (`a / b`) at the given precision.
#[must_use]
pub fn g2_fdiv(pc: u16, a: u64, b: u64) -> F80Out {
    let cw = (CW_FULL & !PC_MASK) | (pc & PC_MASK);
    with_cw(cw, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fdivp",
                "fstp tbyte ptr [{2}]",
                in(reg) &raw const a,
                in(reg) &raw const b,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        let (m, e) = bytes_to_f80(&out);
        F80Out {
            mant: m,
            exp: e,
            status: read_sw(),
        }
    })
}

/// Group 2: square root at the given precision.
#[must_use]
pub fn g2_fsqrt(pc: u16, a: u64) -> F80Out {
    let cw = (CW_FULL & !PC_MASK) | (pc & PC_MASK);
    with_cw(cw, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fsqrt",
                "fstp tbyte ptr [{1}]",
                in(reg) &raw const a,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        let (m, e) = bytes_to_f80(&out);
        F80Out {
            mant: m,
            exp: e,
            status: read_sw(),
        }
    })
}
