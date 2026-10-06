//! x87 probes part 3: group 6 transcendental and remainder instructions.
//! 32-bit x86 only. For these the manuals do not fix every last bit, so the
//! probes return the full 80-bit result plus the status word and the tests
//! assert only what is specified (exactness of square root and remainders,
//! the range rules); the rest is recorded in the dump for the pipeline.

use core::arch::asm;

use crate::x87::{read_sw, with_cw};
use crate::{CW_FULL, F80Out, RC_MASK, bytes_to_f80, f80_to_bytes};

/// Group 6: partial remainder `a % b` (`fprem`: truncated quotient).
/// Inputs are doubles; `a` is the dividend, `b` the divisor.
#[must_use]
pub fn g6_fprem(a: u64, b: u64) -> F80Out {
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fprem",
                "fstp tbyte ptr [{2}]",
                "fstp st(0)",
                in(reg) &raw const b,
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

/// Group 6: partial remainder `a % b` (`fprem1`: round-to-nearest quotient,
/// IEEE-style remainder). Inputs are doubles.
#[must_use]
pub fn g6_fprem1(a: u64, b: u64) -> F80Out {
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fprem1",
                "fstp tbyte ptr [{2}]",
                "fstp st(0)",
                in(reg) &raw const b,
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

/// Group 6: sine. Input is a double.
#[must_use]
pub fn g6_fsin(a: u64) -> F80Out {
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fsin",
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

/// Group 6: cosine. Input is a double.
#[must_use]
pub fn g6_fcos(a: u64) -> F80Out {
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fcos",
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

/// Group 6: sine and cosine together. Returns `(st0, st1)` as left after
/// the instruction (one slot holds the sine, the other the cosine; the
/// `a = 0` case pins the order because sin(0) = +0 and cos(0) = 1).
#[must_use]
pub fn g6_fsincos(a: u64) -> (F80Out, F80Out) {
    with_cw(CW_FULL, || {
        let mut first = [0u8; 10];
        let mut second = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fsincos",
                "fstp tbyte ptr [{1}]",
                "fstp tbyte ptr [{2}]",
                in(reg) &raw const a,
                in(reg) &raw mut first,
                in(reg) &raw mut second,
                options(preserves_flags),
            );
        }
        // The status word is read after both stores; it still carries the
        // C2 range bit from the instruction.
        let status = read_sw();
        let (m0, e0) = bytes_to_f80(&first);
        let (m1, e1) = bytes_to_f80(&second);
        (
            F80Out {
                mant: m0,
                exp: e0,
                status,
            },
            F80Out {
                mant: m1,
                exp: e1,
                status,
            },
        )
    })
}

/// Group 6: arctangent of `y / x` (`fpatan`).
#[must_use]
pub fn g6_fpatan(y: u64, x: u64) -> F80Out {
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fpatan",
                "fstp tbyte ptr [{2}]",
                in(reg) &raw const y,
                in(reg) &raw const x,
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

/// Group 6: scale `a` by `2^n` (`fscale`). Both inputs are doubles.
#[must_use]
pub fn g6_fscale(a: u64, n: u64) -> F80Out {
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "fld qword ptr [{1}]",
                "fscale",
                "fstp tbyte ptr [{2}]",
                "fstp st(0)",
                in(reg) &raw const n,
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

/// Group 6: round to integer (`frndint`) under rounding mode `rc`.
/// Input is a double; the result stays an 80-bit float.
#[must_use]
pub fn g6_frndint(rc: u16, a: u64) -> F80Out {
    let cw = (CW_FULL & !RC_MASK) | (rc & RC_MASK);
    with_cw(cw, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
                "frndint",
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

/// Group 6: load a double and store it as 80-bit (control case showing the
/// exact extended encoding of each input used above).
#[must_use]
pub fn g6_f80_of_f64(a: u64) -> F80Out {
    with_cw(CW_FULL, || {
        let mut out = [0u8; 10];
        unsafe {
            asm!(
                "fld qword ptr [{0}]",
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

/// Group 6: exact 80-bit input passthrough (for building range-rule inputs
/// such as 2^63 and 2^64 exactly).
#[must_use]
pub fn g6_f80_passthrough(mant: u64, exp: u16) -> F80Out {
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
