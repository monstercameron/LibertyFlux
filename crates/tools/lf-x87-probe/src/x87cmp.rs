//! x87 probes part 2: group 3 integer stores, group 4 comparisons and
//! group 5 masked-exception flags. 32-bit x86 only.

use core::arch::asm;

use crate::x87::{read_cw, read_sw, with_cw};
use crate::{CW_FULL, CmpOut, F80Out, I16Out, I32Out, I64Out, RC_MASK, bytes_to_f80, f80_to_bytes};

// ---------------------------------------------------------------------------
// Group 3 (x87 part): float-to-integer stores under each rounding mode.
// `rc` holds raw rounding bits (`RC_NEAR`, `RC_DOWN`, `RC_UP`, `RC_CHOP`).
// Inputs are doubles; out-of-range and NaN inputs give the integer
// indefinite value with the invalid flag set.
// ---------------------------------------------------------------------------

/// Group 3: `fld m64` then `fistp m16` under rounding mode `rc`.
#[must_use]
pub fn g3_fistp_i16(rc: u16, val: u64) -> I16Out {
    let cw = (CW_FULL & !RC_MASK) | (rc & RC_MASK);
    with_cw(cw, || {
        let mut out: u16 = 0;
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fistp word ptr [{1}]",
                in(reg) &raw const val,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        I16Out {
            val: out,
            status: read_sw(),
        }
    })
}

/// Group 3: `fld m64` then `fistp m32` under rounding mode `rc`.
#[must_use]
pub fn g3_fistp_i32(rc: u16, val: u64) -> I32Out {
    let cw = (CW_FULL & !RC_MASK) | (rc & RC_MASK);
    with_cw(cw, || {
        let mut out: u32 = 0;
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fistp dword ptr [{1}]",
                in(reg) &raw const val,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        I32Out {
            val: out,
            status: read_sw(),
        }
    })
}

/// Group 3: `fld m64` then `fistp m64` under rounding mode `rc`.
#[must_use]
pub fn g3_fistp_i64(rc: u16, val: u64) -> I64Out {
    let cw = (CW_FULL & !RC_MASK) | (rc & RC_MASK);
    with_cw(cw, || {
        let mut out: u64 = 0;
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fistp qword ptr [{1}]",
                in(reg) &raw const val,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        I64Out {
            val: out,
            status: read_sw(),
        }
    })
}

/// Group 3: `fld m64` then `fist m32` (store without pop) under mode `rc`.
#[must_use]
pub fn g3_fist_i32(rc: u16, val: u64) -> I32Out {
    let cw = (CW_FULL & !RC_MASK) | (rc & RC_MASK);
    with_cw(cw, || {
        let mut out: u32 = 0;
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fist dword ptr [{1}]",
                "fstp st(0)",
                in(reg) &raw const val,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        I32Out {
            val: out,
            status: read_sw(),
        }
    })
}

/// Group 3: `fld m64` then `fisttp m32` (SSE3 truncating store, ignores the
/// rounding mode). Returns `None` when the processor has no SSE3.
#[must_use]
pub fn g3_fisttp_i32(val: u64) -> Option<I32Out> {
    if !std::arch::is_x86_feature_detected!("sse3") {
        return None;
    }
    Some(with_cw(CW_FULL, || {
        let mut out: u32 = 0;
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fisttp dword ptr [{1}]",
                in(reg) &raw const val,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        I32Out {
            val: out,
            status: read_sw(),
        }
    }))
}

/// Group 3: `fld m64` then `fisttp m64` (SSE3 truncating store, ignores the
/// rounding mode). Returns `None` when the processor has no SSE3.
#[must_use]
pub fn g3_fisttp_i64(val: u64) -> Option<I64Out> {
    if !std::arch::is_x86_feature_detected!("sse3") {
        return None;
    }
    Some(with_cw(CW_FULL, || {
        let mut out: u64 = 0;
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fisttp qword ptr [{1}]",
                in(reg) &raw const val,
                in(reg) &raw mut out,
                options(preserves_flags),
            );
        }
        I64Out {
            val: out,
            status: read_sw(),
        }
    }))
}

// ---------------------------------------------------------------------------
// Group 4: comparison and the status word. All compare `a` against `b`
// (`a` in ST(0), `b` in ST(1)); inputs are doubles.
// ---------------------------------------------------------------------------

/// Group 4: `fucompp` (unordered compare, pop twice); condition bits land in
/// the status word, `eflags` is unused.
#[must_use]
pub fn g4_fucompp(a: u64, b: u64) -> CmpOut {
    with_cw(CW_FULL, || {
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fucompp",
                in(reg) &raw const b,
                in(reg) &raw const a,
                options(preserves_flags),
            );
        }
        CmpOut {
            status: read_sw(),
            eflags: 0,
        }
    })
}

/// Group 4: `fcomp` (ordered compare, pop once) plus a pop to balance.
#[must_use]
pub fn g4_fcomp(a: u64, b: u64) -> CmpOut {
    with_cw(CW_FULL, || {
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fcomp",
                "fstp st(0)",
                in(reg) &raw const b,
                in(reg) &raw const a,
                options(preserves_flags),
            );
        }
        CmpOut {
            status: read_sw(),
            eflags: 0,
        }
    })
}

/// Group 4: `fucomp` (unordered compare, pop once) plus a pop to balance.
#[must_use]
pub fn g4_fucomp(a: u64, b: u64) -> CmpOut {
    with_cw(CW_FULL, || {
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fucomp",
                "fstp st(0)",
                in(reg) &raw const b,
                in(reg) &raw const a,
                options(preserves_flags),
            );
        }
        CmpOut {
            status: read_sw(),
            eflags: 0,
        }
    })
}

/// Group 4: `fcomip` (ordered compare, set EFLAGS, pop once) plus a pop to
/// balance. Returns both EFLAGS and the status word (for the IE flag).
#[must_use]
pub fn g4_fcomip(a: u64, b: u64) -> CmpOut {
    with_cw(CW_FULL, || {
        let eflags: u32;
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fcomip st(0), st(1)",
                "pushfd",
                "pop eax",
                "fstp st(0)",
                in(reg) &raw const b,
                in(reg) &raw const a,
                out("eax") eflags
            );
        }
        CmpOut {
            status: read_sw(),
            eflags,
        }
    })
}

/// Group 4: `fucomip` (unordered compare, set EFLAGS, pop once) plus a pop
/// to balance. Returns both EFLAGS and the status word (for the IE flag).
#[must_use]
pub fn g4_fucomip(a: u64, b: u64) -> CmpOut {
    with_cw(CW_FULL, || {
        let eflags: u32;
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fucomip st(0), st(1)",
                "pushfd",
                "pop eax",
                "fstp st(0)",
                in(reg) &raw const b,
                in(reg) &raw const a,
                out("eax") eflags
            );
        }
        CmpOut {
            status: read_sw(),
            eflags,
        }
    })
}

// ---------------------------------------------------------------------------
// Group 5: masked-exception sticky flags. All run with every exception
// masked (control word 0x037F): each probe returns the full 80-bit result
// plus the status word so the flag and the default result are both visible.
// ---------------------------------------------------------------------------

/// Group 5: `0.0 / 0.0` (invalid operation).
#[must_use]
pub fn g5_fdiv_0_0() -> F80Out {
    let zero: u64 = 0x0000_0000_0000_0000;
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{0}]",
                "fdivp",
                "fstp tbyte ptr [{1}]",
                in(reg) &raw const zero,
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

/// Group 5: `1.0 / 0.0` (zero divide).
#[must_use]
pub fn g5_fdiv_1_0() -> F80Out {
    let one: u64 = 0x3FF0_0000_0000_0000;
    let zero: u64 = 0x0000_0000_0000_0000;
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fdivp",
                "fstp tbyte ptr [{2}]",
                in(reg) &raw const one,
                in(reg) &raw const zero,
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

/// Group 5: `1.0 / 3.0` (inexact result).
#[must_use]
pub fn g5_fdiv_1_3() -> F80Out {
    let one: u64 = 0x3FF0_0000_0000_0000;
    let three: u64 = 0x4008_0000_0000_0000;
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fdivp",
                "fstp tbyte ptr [{2}]",
                in(reg) &raw const one,
                in(reg) &raw const three,
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

/// Group 5: `sqrt(-1.0)` (invalid operation).
#[must_use]
pub fn g5_fsqrt_neg1() -> F80Out {
    let neg1: u64 = 0xBFF0_0000_0000_0000;
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fsqrt",
                "fstp tbyte ptr [{1}]",
                in(reg) &raw const neg1,
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

/// Group 5: maximum finite extended value doubled (overflow: the
/// exponent range stays 80-bit whatever the precision field says, so only
/// an 80-bit input near 1e4932 can overflow).
#[must_use]
pub fn g5_fadd_overflow() -> F80Out {
    // Largest finite extended value: exponent 0x7FFE, all mantissa bits set.
    let big = f80_to_bytes(0xFFFF_FFFF_FFFF_FFFF, 0x7FFE);
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld tbyte ptr [{0}]",
                "fld tbyte ptr [{0}]",
                "faddp",
                "fstp tbyte ptr [{1}]",
                in(reg) &raw const big,
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

/// Group 5: `2^-16000 * 2^-1000` (underflow: the true product 2^-17000 is
/// below the extended minimum 2^-16382, so even the 80-bit range overflows).
/// Inputs are given as 80-bit values because no double is that small.
#[must_use]
pub fn g5_fmul_underflow() -> F80Out {
    // 2^-16000: exponent word 16383-16000 = 383.
    let a = crate::f80_to_bytes(0x8000_0000_0000_0000, 383);
    // 2^-1000: exponent word 16383-1000 = 15383.
    let b = crate::f80_to_bytes(0x8000_0000_0000_0000, 15383);
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld tbyte ptr [{0}]",
                "fld tbyte ptr [{1}]",
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

/// Group 5: `fld` of a denormal single (records whether the denormal flag
/// is set at all; the rule for `fld` raising it is checked, not assumed).
#[must_use]
pub fn g5_fld_denormal_single() -> F80Out {
    let den: u32 = 0x0000_0001;
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld dword ptr [{0}]",
                "fstp tbyte ptr [{1}]",
                in(reg) &raw const den,
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

/// Read back the ambient control word (used by the dump header to show the
/// word the process runs with outside probes).
#[must_use]
pub fn ambient_cw() -> u16 {
    read_cw()
}
