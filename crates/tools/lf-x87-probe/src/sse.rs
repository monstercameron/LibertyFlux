//! SSE scalar probes: group 7 (NaN propagation, min/max, conversions)
//! plus the vector float-to-integer conversions of group 3. 32-bit x86
//! only. Every probe installs a known MXCSR (default 0x1F80 unless stated),
//! runs one instruction, and reads MXCSR back.

use core::arch::asm;

use crate::{MXCSR_DEFAULT, Sse32Out, Sse64Out};

/// Read MXCSR.
fn read_mxcsr() -> u32 {
    let mut mx: u32 = 0;
    unsafe {
        asm!(
            "stmxcsr dword ptr [{0}]",
            in(reg) &raw mut mx,
            options(preserves_flags),
        );
    }
    mx
}

/// Load MXCSR.
fn write_mxcsr(mx: u32) {
    unsafe {
        asm!(
            "ldmxcsr dword ptr [{0}]",
            in(reg) &raw const mx,
            options(preserves_flags),
        );
    }
}

/// Run `f` under MXCSR `mx`, restoring the ambient register afterwards.
/// Writing the full register also clears the sticky flags.
fn with_mxcsr<R>(mx: u32, f: impl FnOnce() -> R) -> R {
    let saved = read_mxcsr();
    write_mxcsr(mx);
    let out = f();
    write_mxcsr(saved);
    out
}

/// One scalar-single arithmetic op with a memory source operand.
macro_rules! ss_arith {
    ($name:ident, $insn:literal) => {
        /// Group 7: scalar-single `a op b` (masked, round-nearest).
        #[must_use]
        pub fn $name(a: u32, b: u32) -> Sse32Out {
            with_mxcsr(MXCSR_DEFAULT, || {
                let mut out: u32 = 0;
                unsafe {
                    asm!(
                        "movss xmm0, dword ptr [{0}]",
                        $insn,
                        "movss dword ptr [{2}], xmm0",
                        in(reg) &raw const a,
                        in(reg) &raw const b,
                        in(reg) &raw mut out,
                        out("xmm0") _,
                        options(preserves_flags),
                    );
                }
                Sse32Out {
                    bits: out,
                    mxcsr: read_mxcsr(),
                }
            })
        }
    };
}

ss_arith!(g7_addss, "addss xmm0, dword ptr [{1}]");
ss_arith!(g7_subss, "subss xmm0, dword ptr [{1}]");
ss_arith!(g7_mulss, "mulss xmm0, dword ptr [{1}]");
ss_arith!(g7_divss, "divss xmm0, dword ptr [{1}]");
ss_arith!(g7_minss, "minss xmm0, dword ptr [{1}]");
ss_arith!(g7_maxss, "maxss xmm0, dword ptr [{1}]");

/// One scalar-double arithmetic op with a memory source operand.
macro_rules! sd_arith {
    ($name:ident, $insn:literal) => {
        /// Group 7: scalar-double `a op b` (masked, round-nearest).
        #[must_use]
        pub fn $name(a: u64, b: u64) -> Sse64Out {
            with_mxcsr(MXCSR_DEFAULT, || {
                let mut out: u64 = 0;
                unsafe {
                    asm!(
                        "movsd xmm0, qword ptr [{0}]",
                        $insn,
                        "movsd qword ptr [{2}], xmm0",
                        in(reg) &raw const a,
                        in(reg) &raw const b,
                        in(reg) &raw mut out,
                        out("xmm0") _,
                        options(preserves_flags),
                    );
                }
                Sse64Out {
                    bits: out,
                    mxcsr: read_mxcsr(),
                }
            })
        }
    };
}

sd_arith!(g7_addsd, "addsd xmm0, qword ptr [{1}]");
sd_arith!(g7_mulsd, "mulsd xmm0, qword ptr [{1}]");
sd_arith!(g7_minsd, "minsd xmm0, qword ptr [{1}]");
sd_arith!(g7_maxsd, "maxsd xmm0, qword ptr [{1}]");

/// Group 7: scalar-single square root (masked, round-nearest).
#[must_use]
pub fn g7_sqrtss(a: u32) -> Sse32Out {
    with_mxcsr(MXCSR_DEFAULT, || {
        let mut out: u32 = 0;
        unsafe {
            asm!(
                "sqrtss xmm0, dword ptr [{0}]",
                "movss dword ptr [{1}], xmm0",
                in(reg) &raw const a,
                in(reg) &raw mut out,
                out("xmm0") _,
                options(preserves_flags),
            );
        }
        Sse32Out {
            bits: out,
            mxcsr: read_mxcsr(),
        }
    })
}

/// Group 7: scalar-double square root (masked, round-nearest).
#[must_use]
pub fn g7_sqrtsd(a: u64) -> Sse64Out {
    with_mxcsr(MXCSR_DEFAULT, || {
        let mut out: u64 = 0;
        unsafe {
            asm!(
                "sqrtsd xmm0, qword ptr [{0}]",
                "movsd qword ptr [{1}], xmm0",
                in(reg) &raw const a,
                in(reg) &raw mut out,
                out("xmm0") _,
                options(preserves_flags),
            );
        }
        Sse64Out {
            bits: out,
            mxcsr: read_mxcsr(),
        }
    })
}

/// Group 7: convert scalar-single to scalar-double.
#[must_use]
pub fn g7_cvtss2sd(a: u32) -> Sse64Out {
    with_mxcsr(MXCSR_DEFAULT, || {
        let mut out: u64 = 0;
        unsafe {
            asm!(
                "cvtss2sd xmm0, dword ptr [{0}]",
                "movsd qword ptr [{1}], xmm0",
                in(reg) &raw const a,
                in(reg) &raw mut out,
                out("xmm0") _,
                options(preserves_flags),
            );
        }
        Sse64Out {
            bits: out,
            mxcsr: read_mxcsr(),
        }
    })
}

/// Group 7: convert scalar-double to scalar-single (round-nearest).
#[must_use]
pub fn g7_cvtsd2ss(a: u64) -> Sse32Out {
    with_mxcsr(MXCSR_DEFAULT, || {
        let mut out: u32 = 0;
        unsafe {
            asm!(
                "cvtsd2ss xmm0, qword ptr [{0}]",
                "movss dword ptr [{1}], xmm0",
                in(reg) &raw const a,
                in(reg) &raw mut out,
                out("xmm0") _,
                options(preserves_flags),
            );
        }
        Sse32Out {
            bits: out,
            mxcsr: read_mxcsr(),
        }
    })
}

/// Group 7: convert 32-bit integer to scalar-single.
#[must_use]
pub fn g7_cvtsi2ss(a: i32) -> Sse32Out {
    with_mxcsr(MXCSR_DEFAULT, || {
        let mut out: u32 = 0;
        unsafe {
            asm!(
                "cvtsi2ss xmm0, dword ptr [{0}]",
                "movss dword ptr [{1}], xmm0",
                in(reg) &raw const a,
                in(reg) &raw mut out,
                out("xmm0") _,
                options(preserves_flags),
            );
        }
        Sse32Out {
            bits: out,
            mxcsr: read_mxcsr(),
        }
    })
}

/// Group 7: convert 32-bit integer to scalar-double.
#[must_use]
pub fn g7_cvtsi2sd(a: i32) -> Sse64Out {
    with_mxcsr(MXCSR_DEFAULT, || {
        let mut out: u64 = 0;
        unsafe {
            asm!(
                "cvtsi2sd xmm0, dword ptr [{0}]",
                "movsd qword ptr [{1}], xmm0",
                in(reg) &raw const a,
                in(reg) &raw mut out,
                out("xmm0") _,
                options(preserves_flags),
            );
        }
        Sse64Out {
            bits: out,
            mxcsr: read_mxcsr(),
        }
    })
}

/// Group 3: convert scalar-single to 32-bit integer using the MXCSR
/// rounding mode in `mxcsr` (caller sets bits 13-14).
#[must_use]
pub fn g3_cvtss2si(mxcsr: u32, a: u32) -> (i32, u32) {
    with_mxcsr(mxcsr, || {
        let out: i32;
        unsafe {
            asm!(
                "cvtss2si {0}, dword ptr [{1}]",
                out(reg) out,
                in(reg) &raw const a,
                options(preserves_flags),
            );
        }
        (out, read_mxcsr())
    })
}

/// Group 3: convert scalar-double to 32-bit integer using the MXCSR
/// rounding mode in `mxcsr` (caller sets bits 13-14).
#[must_use]
pub fn g3_cvtsd2si(mxcsr: u32, a: u64) -> (i32, u32) {
    with_mxcsr(mxcsr, || {
        let out: i32;
        unsafe {
            asm!(
                "cvtsd2si {0}, qword ptr [{1}]",
                out(reg) out,
                in(reg) &raw const a,
                options(preserves_flags),
            );
        }
        (out, read_mxcsr())
    })
}

/// Group 3: convert scalar-single to 32-bit integer with truncation.
#[must_use]
pub fn g3_cvttss2si(a: u32) -> (i32, u32) {
    with_mxcsr(MXCSR_DEFAULT, || {
        let out: i32;
        unsafe {
            asm!(
                "cvttss2si {0}, dword ptr [{1}]",
                out(reg) out,
                in(reg) &raw const a,
                options(preserves_flags),
            );
        }
        (out, read_mxcsr())
    })
}

/// Group 3: convert scalar-double to 32-bit integer with truncation.
#[must_use]
pub fn g3_cvttsd2si(a: u64) -> (i32, u32) {
    with_mxcsr(MXCSR_DEFAULT, || {
        let out: i32;
        unsafe {
            asm!(
                "cvttsd2si {0}, qword ptr [{1}]",
                out(reg) out,
                in(reg) &raw const a,
                options(preserves_flags),
            );
        }
        (out, read_mxcsr())
    })
}

/// Group 7: scalar-single add under an explicit MXCSR (used for the
/// flush-to-zero pair: same inputs with FTZ off and on).
#[must_use]
pub fn g7_addss_mxcsr(mxcsr: u32, a: u32, b: u32) -> Sse32Out {
    with_mxcsr(mxcsr, || {
        let mut out: u32 = 0;
        unsafe {
            asm!(
                "movss xmm0, dword ptr [{0}]",
                "addss xmm0, dword ptr [{1}]",
                "movss dword ptr [{2}], xmm0",
                in(reg) &raw const a,
                in(reg) &raw const b,
                in(reg) &raw mut out,
                out("xmm0") _,
                options(preserves_flags),
            );
        }
        Sse32Out {
            bits: out,
            mxcsr: read_mxcsr(),
        }
    })
}

/// Read back the ambient MXCSR (used by the dump header).
#[must_use]
pub fn ambient_mxcsr() -> u32 {
    read_mxcsr()
}
