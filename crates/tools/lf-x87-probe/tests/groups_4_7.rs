//! Documented-behaviour tests, groups 4-7. Each test states the Intel
//! manual's rule in words and asserts the raw bits a real x86 processor
//! produces. x86 only; on other targets this file compiles to nothing.
#![cfg(target_arch = "x86")]

use lf_x87_probe::sse;
use lf_x87_probe::x87;
use lf_x87_probe::x87cmp;
use lf_x87_probe::x87trans;
use lf_x87_probe::{
    MXCSR_DEFAULT, MXCSR_IE, MXCSR_PE, PC_64, RC_CHOP, RC_DOWN, RC_NEAR, RC_UP, SW_C0, SW_C1,
    SW_C2, SW_C3, SW_IE, SW_OE, SW_PE, SW_UE, SW_ZE, emulation_details, is_emulated,
};

fn is_qnan32(v: u32) -> bool {
    v & 0x7F80_0000 == 0x7F80_0000 && v & 0x007F_FFFF != 0 && v & 0x0040_0000 != 0
}

fn is_qnan64(v: u64) -> bool {
    v & 0x7FF0_0000_0000_0000 == 0x7FF0_0000_0000_0000
        && v & 0x000F_FFFF_FFFF_FFFF != 0
        && v & 0x0008_0000_0000_0000 != 0
}

/// Condition bits as `(C3, C2, C0)`.
fn cond(status: u16) -> (bool, bool, bool) {
    (
        status & SW_C3 != 0,
        status & SW_C2 != 0,
        status & SW_C0 != 0,
    )
}

const F64_ONE: u64 = 0x3FF0_0000_0000_0000;
const F64_TWO: u64 = 0x4000_0000_0000_0000;
const F64_QNAN: u64 = 0x7FF8_0000_0000_0000;
const F64_SNAN: u64 = 0x7FF0_0000_0000_0001;
const F64_PZERO: u64 = 0x0000_0000_0000_0000;
const F64_NZERO: u64 = 0x8000_0000_0000_0000;
const F64_PINF: u64 = 0x7FF0_0000_0000_0000;

// ---------------------------------------------------------------------------
// Group 4.
// ---------------------------------------------------------------------------

#[test]
fn g4_fucompp_condition_bits() {
    // KNOWN-EMULATOR-DIFF (g4-fucompp-snan-IE): unordered bits set but IE clear for sNaN.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff("g4-fucompp-snan-IE", "FUCOMPP sNaN unordered but IE clear");
    // Rule: FUCOMPP sets C3,C2,C0 to 000 when ST(0) is greater, 001 when
    // less, 100 when equal and 111 when unordered (either operand NaN). A
    // signalling NaN also raises invalid; a quiet NaN does not.
    for (a, b, expected, ie, tag) in [
        (F64_TWO, F64_ONE, (false, false, false), false, "greater"),
        (F64_ONE, F64_TWO, (false, false, true), false, "less"),
        (F64_ONE, F64_ONE, (true, false, false), false, "equal"),
        (F64_QNAN, F64_ONE, (true, true, true), false, "qNaN first"),
        (F64_ONE, F64_QNAN, (true, true, true), false, "qNaN second"),
        (F64_SNAN, F64_ONE, (true, true, true), true, "sNaN"),
        (
            F64_PZERO,
            F64_NZERO,
            (true, false, false),
            false,
            "zeros equal",
        ),
        (
            F64_PINF,
            F64_PINF,
            (true, false, false),
            false,
            "infs equal",
        ),
    ] {
        let o = x87cmp::g4_fucompp(a, b);
        assert_eq!(cond(o.status), expected, "{tag}");
        assert_eq!(o.status & SW_IE != 0, ie, "{tag} IE");
    }
}

#[test]
fn g4_fcomp_fucomp_qnan_ie() {
    // KNOWN-EMULATOR-DIFF (g4-fcomp-qnan-IE): FCOMP with qNaN leaves IE clear (FUCOMP fine).
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff("g4-fcomp-qnan-IE", "FCOMP qNaN unordered but IE clear");
    // Rule: FCOMP and FUCOMP report the same condition bits, but FCOMP
    // raises invalid when either operand is a quiet NaN while FUCOMP stays
    // silent and only reports unordered.
    let o = x87cmp::g4_fcomp(F64_ONE, F64_ONE);
    assert_eq!(cond(o.status), (true, false, false));
    assert_eq!(o.status & SW_IE, 0);
    let o = x87cmp::g4_fcomp(F64_QNAN, F64_ONE);
    assert_eq!(cond(o.status), (true, true, true), "FCOMP qNaN unordered");
    assert_eq!(o.status & SW_IE, SW_IE, "FCOMP qNaN raises");
    let o = x87cmp::g4_fucomp(F64_ONE, F64_ONE);
    assert_eq!(cond(o.status), (true, false, false));
    let o = x87cmp::g4_fucomp(F64_QNAN, F64_ONE);
    assert_eq!(cond(o.status), (true, true, true), "FUCOMP qNaN unordered");
    assert_eq!(o.status & SW_IE, 0, "FUCOMP qNaN silent");
}

#[test]
fn g4_fcomip_fucomip_flags() {
    // KNOWN-EMULATOR-DIFF (g4-fcomip-IE): FCOMIP IE clear for qNaN/sNaN, FUCOMIP for sNaN.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff("g4-fcomip-IE", "FCOMIP/FUCOMIP IE clear for NaN");
    // Rule: the compare-and-set-flags forms write ZF, PF and CF as 000
    // (greater), 001 (less, CF set), 100 (equal, ZF set) and 111
    // (unordered). FCOMIP raises invalid for a quiet NaN; FUCOMIP does not;
    // both raise for a signalling NaN.
    for (a, b, expected, tag) in [
        (F64_TWO, F64_ONE, 0x00u32, "greater"),
        (F64_ONE, F64_TWO, 0x01u32, "less"),
        (F64_ONE, F64_ONE, 0x40u32, "equal"),
        (F64_QNAN, F64_ONE, 0x45u32, "qNaN"),
        (F64_SNAN, F64_ONE, 0x45u32, "sNaN"),
    ] {
        let o = x87cmp::g4_fcomip(a, b);
        assert_eq!(o.eflags & 0x45, expected, "FCOMIP {tag}");
        let o = x87cmp::g4_fucomip(a, b);
        assert_eq!(o.eflags & 0x45, expected, "FUCOMIP {tag}");
    }
    let o = x87cmp::g4_fcomip(F64_QNAN, F64_ONE);
    assert_eq!(o.status & SW_IE, SW_IE, "FCOMIP qNaN raises");
    let o = x87cmp::g4_fucomip(F64_QNAN, F64_ONE);
    assert_eq!(o.status & SW_IE, 0, "FUCOMIP qNaN silent");
    let o = x87cmp::g4_fcomip(F64_SNAN, F64_ONE);
    assert_eq!(o.status & SW_IE, SW_IE, "FCOMIP sNaN raises");
    let o = x87cmp::g4_fucomip(F64_SNAN, F64_ONE);
    assert_eq!(o.status & SW_IE, SW_IE, "FUCOMIP sNaN raises");
    // The condition-bit constants cover the whole set used above.
    assert_eq!(SW_C1, 0x0200);
}

// ---------------------------------------------------------------------------
// Group 5.
// ---------------------------------------------------------------------------

#[test]
fn g5_masked_flags_and_defaults() {
    // KNOWN-EMULATOR-DIFF (g5-invalid-default-flags): masked invalid gives +qNaN, PE/OE/UE
    // never set, fsqrt(-1) leaves IE clear. ZE (1/0) and IE (0/0) are correct.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff(
        "g5-invalid-default-flags",
        "+qNaN not indefinite; PE/OE/UE never set",
    );
    // Rule: with every exception masked, an invalid operation returns the
    // real indefinite quiet NaN and sets IE; zero divide returns a signed
    // infinity and sets ZE; a rounded result sets PE; overflow returns
    // infinity and sets OE; underflow sets UE.
    let o = x87cmp::g5_fdiv_0_0();
    assert_eq!(
        (o.mant, o.exp),
        (0xC000_0000_0000_0000, 0xFFFF),
        "0/0 indefinite"
    );
    assert_eq!(o.status & SW_IE, SW_IE, "0/0 IE");
    let o = x87cmp::g5_fdiv_1_0();
    assert_eq!(
        (o.mant, o.exp),
        (0x8000_0000_0000_0000, 0x7FFF),
        "1/0 is +inf"
    );
    assert_eq!(o.status & SW_ZE, SW_ZE, "1/0 ZE");
    assert_eq!(o.status & SW_IE, 0, "1/0 no IE");
    let o = x87cmp::g5_fdiv_1_3();
    assert_eq!(
        (o.mant, o.exp),
        (0xAAAA_AAAA_AAAA_AAAB, 0x3FFD),
        "1/3 value"
    );
    assert_eq!(o.status & SW_PE, SW_PE, "1/3 PE");
    assert_eq!(o.status & (SW_IE | SW_ZE), 0, "1/3 no IE/ZE");
    let o = x87cmp::g5_fsqrt_neg1();
    assert_eq!(
        (o.mant, o.exp),
        (0xC000_0000_0000_0000, 0xFFFF),
        "sqrt(-1) indefinite"
    );
    assert_eq!(o.status & SW_IE, SW_IE, "sqrt(-1) IE");
    let o = x87cmp::g5_fadd_overflow();
    assert_eq!(
        (o.mant, o.exp),
        (0x8000_0000_0000_0000, 0x7FFF),
        "overflow is +inf"
    );
    assert_eq!(o.status & SW_OE, SW_OE, "overflow OE");
    let o = x87cmp::g5_fmul_underflow();
    assert_eq!(
        (o.mant, o.exp),
        (0x0000_0000_0000_0000, 0x0000),
        "underflow flushes to +0"
    );
    assert_eq!(o.status & SW_UE, SW_UE, "underflow UE");
    // A denormal single loads to its exact extended value; whether the
    // denormal flag is set is recorded in the dump, not asserted here.
    let o = x87cmp::g5_fld_denormal_single();
    assert_eq!(
        (o.mant, o.exp),
        (0x8000_0000_0000_0000, 0x3F6A),
        "2^-149 exact"
    );
}

// ---------------------------------------------------------------------------
// Group 6. Only specified behaviour is asserted; the rest is recorded.
// ---------------------------------------------------------------------------

#[test]
fn g6_fprem_exact_and_incomplete() {
    // Rule: FPREM computes the exact remainder with a truncated quotient;
    // C2 is clear when the reduction finished and set when the quotient has
    // more than 63 bits so the instruction must run again.
    let o = x87trans::g6_fprem(0x4016_0000_0000_0000, F64_TWO);
    assert_eq!(
        (o.mant, o.exp),
        (0xC000_0000_0000_0000, 0x3FFF),
        "5.5 % 2 = 1.5"
    );
    assert_eq!(o.status & SW_C2, 0, "complete, C2 clear");
    let o = x87trans::g6_fprem(0x4415_AF1D_78B5_8C40, F64_TWO);
    assert_eq!(o.status & SW_C2, SW_C2, "1e20 % 2 needs another pass");
}

#[test]
fn g6_fprem1_ieee_remainder() {
    // Rule: FPREM1 computes the exact remainder with the quotient rounded
    // to nearest (ties to even), the IEEE-style remainder.
    let o = x87trans::g6_fprem1(0x4016_0000_0000_0000, F64_TWO);
    assert_eq!(
        (o.mant, o.exp),
        (0x8000_0000_0000_0000, 0xBFFE),
        "5.5 rem 2 = -0.5"
    );
    assert_eq!(o.status & SW_C2, 0);
    let o = x87trans::g6_fprem1(0x401C_0000_0000_0000, F64_TWO);
    assert_eq!(
        (o.mant, o.exp),
        (0x8000_0000_0000_0000, 0xBFFF),
        "7 rem 2 = -1"
    );
    assert_eq!(o.status & SW_C2, 0);
}

#[test]
fn g6_fsqrt_exact_cases() {
    // Rule: square root is correctly rounded, so perfect squares, zero and
    // infinity are exact.
    let o = x87::g2_fsqrt(PC_64, 0x4010_0000_0000_0000);
    assert_eq!(
        (o.mant, o.exp),
        (0x8000_0000_0000_0000, 0x4000),
        "sqrt(4) = 2"
    );
    assert_eq!(o.status & SW_PE, 0, "sqrt(4) exact");
    let o = x87::g2_fsqrt(PC_64, 0x0000_0000_0000_0000);
    assert_eq!((o.mant, o.exp), (0, 0), "sqrt(+0) = +0");
    let o = x87::g2_fsqrt(PC_64, F64_PINF);
    assert_eq!(
        (o.mant, o.exp),
        (0x8000_0000_0000_0000, 0x7FFF),
        "sqrt(+inf) = +inf"
    );
}

#[test]
fn g6_trig_range_rule() {
    // Rule: when the absolute value of the operand exceeds 2^63, sine and
    // cosine set C2 and leave the operand unchanged; in-range results are
    // recorded in the dump because the manuals do not fix every last bit.
    let o = x87trans::g6_fsin(0x0000_0000_0000_0000);
    assert_eq!((o.mant, o.exp), (0, 0), "sin(+0) = +0");
    assert_eq!(o.status & SW_C2, 0);
    let o = x87trans::g6_fsin(0x43F0_0000_0000_0000);
    assert_eq!(
        (o.mant, o.exp),
        (0x8000_0000_0000_0000, 0x403F),
        "2^64 unchanged"
    );
    assert_eq!(o.status & SW_C2, SW_C2, "out of range");
    let o = x87trans::g6_fsin(F64_ONE);
    assert_eq!(o.status & SW_C2, 0, "sin(1) in range; value recorded");
    let o = x87trans::g6_fcos(0x0000_0000_0000_0000);
    assert_eq!(
        (o.mant, o.exp),
        (0x8000_0000_0000_0000, 0x3FFF),
        "cos(0) = 1"
    );
    assert_eq!(o.status & SW_C2, 0);
    let o = x87trans::g6_fcos(0x43F0_0000_0000_0000);
    assert_eq!(
        (o.mant, o.exp),
        (0x8000_0000_0000_0000, 0x403F),
        "2^64 unchanged"
    );
    assert_eq!(o.status & SW_C2, SW_C2, "out of range");
}

#[test]
fn g6_fsincos_fpatan_fscale() {
    // Rule: FSINCOS returns the sine and the cosine (the zero case shows
    // one slot +0 and the other 1); FPATAN(0, 1) is +0; FSCALE with an
    // integer scale only moves the exponent, so it is exact.
    let (s0, s1) = x87trans::g6_fsincos(0x0000_0000_0000_0000);
    let pair = ((s0.mant, s0.exp), (s1.mant, s1.exp));
    assert!(
        pair == ((0, 0), (0x8000_0000_0000_0000, 0x3FFF))
            || pair == ((0x8000_0000_0000_0000, 0x3FFF), (0, 0)),
        "fsincos(0) is {{+0, 1}} in some order, got {pair:?}",
    );
    let (s0, _) = x87trans::g6_fsincos(F64_ONE);
    assert_eq!(s0.status & SW_C2, 0, "fsincos(1) in range; values recorded");
    let o = x87trans::g6_fpatan(0x0000_0000_0000_0000, F64_ONE);
    assert_eq!((o.mant, o.exp), (0, 0), "atan(0/1) = +0");
    // Nontrivial arctangents are recorded; they must at least be finite.
    for (y, x) in [(F64_ONE, F64_ONE), (F64_ONE, 0x0000_0000_0000_0000)] {
        let o = x87trans::g6_fpatan(y, x);
        assert!(
            (o.exp & 0x7FFF) != 0 && (o.exp & 0x7FFF) != 0x7FFF,
            "finite recorded value"
        );
        assert_eq!(o.mant >> 63, 1, "normal recorded value");
    }
    let o = x87trans::g6_fscale(0x3FF8_0000_0000_0000, F64_TWO);
    assert_eq!(
        (o.mant, o.exp),
        (0xC000_0000_0000_0000, 0x4001),
        "1.5 * 4 = 6"
    );
    let o = x87trans::g6_fscale(F64_ONE, 0xBFF0_0000_0000_0000);
    assert_eq!(
        (o.mant, o.exp),
        (0x8000_0000_0000_0000, 0x3FFE),
        "1 / 2 = 0.5"
    );
}

#[test]
fn g6_frndint_rounding_modes() {
    // Rule: FRNDINT rounds to an integer using the rounding field with the
    // same four meanings as FISTP, keeping a float result.
    for (rc, p, n) in [
        (
            RC_NEAR,
            (0x8000_0000_0000_0000u64, 0x4000u16),
            (0x8000_0000_0000_0000u64, 0xC000u16),
        ),
        (
            RC_DOWN,
            (0x8000_0000_0000_0000, 0x4000),
            (0xC000_0000_0000_0000, 0xC000),
        ),
        (
            RC_UP,
            (0xC000_0000_0000_0000, 0x4000),
            (0x8000_0000_0000_0000, 0xC000),
        ),
        (
            RC_CHOP,
            (0x8000_0000_0000_0000, 0x4000),
            (0x8000_0000_0000_0000, 0xC000),
        ),
    ] {
        let o = x87trans::g6_frndint(rc, 0x4004_0000_0000_0000);
        assert_eq!((o.mant, o.exp), p, "rc {rc:#06x} +2.5");
        let o = x87trans::g6_frndint(rc, 0xC004_0000_0000_0000);
        assert_eq!((o.mant, o.exp), n, "rc {rc:#06x} -2.5");
    }
}

// ---------------------------------------------------------------------------
// Group 7.
// ---------------------------------------------------------------------------

#[test]
fn g7_arith_nan_quiet() {
    // KNOWN-EMULATOR-DIFF (g7-arith-snan-IE): ADDSS/SUBSS/MULSS leave IE clear for sNaN.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff(
        "g7-arith-snan-IE",
        "ADDSS/SUBSS/MULSS sNaN IE clear (DIVSS sets it)",
    );
    // Rule: scalar SSE arithmetic quiets a signalling NaN input (raising
    // invalid) and propagates a quiet NaN silently; the result is always a
    // quiet NaN. Which operand's payload wins is recorded for the pipeline.
    for (a, b, ie, tag) in [
        (0x7FC0_1111u32, 0x3F80_0000u32, false, "qNaN first"),
        (0x3F80_0000u32, 0x7FC0_2222u32, false, "qNaN second"),
        (0x7FC0_1111u32, 0x7FC0_2222u32, false, "qNaN both"),
        (0x7F80_1111u32, 0x3F80_0000u32, true, "sNaN first"),
    ] {
        for (o, op) in [
            (sse::g7_addss(a, b), "addss"),
            (sse::g7_subss(a, b), "subss"),
            (sse::g7_mulss(a, b), "mulss"),
            (sse::g7_divss(a, b), "divss"),
        ] {
            assert!(
                is_qnan32(o.bits),
                "{op} {tag}: quiet NaN out, got {:#010x}",
                o.bits
            );
            assert_eq!(o.mxcsr & MXCSR_IE != 0, ie, "{op} {tag} IE");
        }
    }
    let o = sse::g7_addsd(0x7FF8_0000_0001_1111, 0x7FF8_0000_0002_2222);
    assert!(is_qnan64(o.bits), "addsd propagates a quiet NaN");
    assert_eq!(o.mxcsr & MXCSR_IE, 0);
    let o = sse::g7_mulsd(0x7FF8_0000_0001_1111, 0x7FF8_0000_0002_2222);
    assert!(is_qnan64(o.bits), "mulsd propagates a quiet NaN");
    assert_eq!(o.mxcsr & MXCSR_IE, 0);
}

#[test]
fn g7_sqrt_nan_neg() {
    // KNOWN-EMULATOR-DIFF (g7-sqrtss-IE): SQRTSS(-1) returns indefinite but IE clear.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff("g7-sqrtss-IE", "SQRTSS(-1) indefinite but IE clear");
    // Rule: a scalar square root of a quiet NaN is a quiet NaN without an
    // exception; of a negative it is the indefinite quiet NaN with invalid.
    let o = sse::g7_sqrtss(0x7FC0_1111);
    assert!(is_qnan32(o.bits), "sqrtss(qNaN) is qNaN");
    assert_eq!(o.mxcsr & MXCSR_IE, 0);
    let o = sse::g7_sqrtss(0xBF80_0000);
    assert_eq!(o.bits, 0xFFC0_0000, "sqrtss(-1) indefinite");
    assert_eq!(o.mxcsr & MXCSR_IE, MXCSR_IE, "sqrtss(-1) IE");
    let o = sse::g7_sqrtsd(0x7FF8_0000_0001_1111);
    assert!(is_qnan64(o.bits), "sqrtsd(qNaN) is qNaN");
    assert_eq!(o.mxcsr & MXCSR_IE, 0);
}

#[test]
fn g7_minmax_nan_zeros() {
    // KNOWN-EMULATOR-DIFF (g7-minmax-zeros): mixed zeros return the 2nd operand, not -0/+0.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff("g7-minmax-zeros", "MIN/MAX mixed zeros return 2nd operand");
    // Rule: scalar minimum and maximum return the second (source) operand
    // when either operand is a NaN, without raising for a quiet NaN; signed
    // zeros compare equal in value but min keeps -0 and max keeps +0.
    for (a, b, expected, tag) in [
        (0x7FC0_1111u32, 0x3F80_0000u32, 0x3F80_0000u32, "NaN first"),
        (0x3F80_0000u32, 0x7FC0_2222u32, 0x7FC0_2222u32, "NaN second"),
        (0x8000_0000u32, 0x0000_0000u32, 0x8000_0000u32, "-0,+0"),
        (0x0000_0000u32, 0x8000_0000u32, 0x8000_0000u32, "+0,-0"),
    ] {
        let o = sse::g7_minss(a, b);
        assert_eq!(o.bits, expected, "minss {tag}");
        assert_eq!(o.mxcsr & MXCSR_IE, 0, "minss {tag} no IE");
    }
    for (a, b, expected, tag) in [
        (0x7FC0_1111u32, 0x3F80_0000u32, 0x3F80_0000u32, "NaN first"),
        (0x3F80_0000u32, 0x7FC0_2222u32, 0x7FC0_2222u32, "NaN second"),
        (0x8000_0000u32, 0x0000_0000u32, 0x0000_0000u32, "-0,+0"),
        (0x0000_0000u32, 0x8000_0000u32, 0x0000_0000u32, "+0,-0"),
    ] {
        let o = sse::g7_maxss(a, b);
        assert_eq!(o.bits, expected, "maxss {tag}");
        assert_eq!(o.mxcsr & MXCSR_IE, 0, "maxss {tag} no IE");
    }
    for (a, b, expected, tag) in [
        (
            0x7FF8_0000_0001_1111u64,
            0x3FF0_0000_0000_0000u64,
            0x3FF0_0000_0000_0000u64,
            "NaN first",
        ),
        (
            0x3FF0_0000_0000_0000u64,
            0x7FF8_0000_0002_2222u64,
            0x7FF8_0000_0002_2222u64,
            "NaN second",
        ),
        (F64_NZERO, F64_PZERO, F64_NZERO, "-0,+0"),
        (F64_PZERO, F64_NZERO, F64_NZERO, "+0,-0"),
    ] {
        let o = sse::g7_minsd(a, b);
        assert_eq!(o.bits, expected, "minsd {tag}");
    }
    for (a, b, expected, tag) in [
        (
            0x7FF8_0000_0001_1111u64,
            0x3FF0_0000_0000_0000u64,
            0x3FF0_0000_0000_0000u64,
            "NaN first",
        ),
        (
            0x3FF0_0000_0000_0000u64,
            0x7FF8_0000_0002_2222u64,
            0x7FF8_0000_0002_2222u64,
            "NaN second",
        ),
        (F64_NZERO, F64_PZERO, F64_PZERO, "-0,+0"),
        (F64_PZERO, F64_NZERO, F64_PZERO, "+0,-0"),
    ] {
        let o = sse::g7_maxsd(a, b);
        assert_eq!(o.bits, expected, "maxsd {tag}");
    }
}

#[test]
fn g7_conversions_nan_inf() {
    // KNOWN-EMULATOR-DIFF (g7-cvt-flags): sNaN IE clear; PE/OE/UE never set; values correct.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff("g7-cvt-flags", "CVT sNaN IE clear; PE/OE/UE never set");
    // Rule: scalar conversions turn NaN into NaN (a signalling input raises
    // invalid and is quieted); infinities convert exactly; a rounded result
    // sets inexact; overflow gives infinity with overflow set; an input far
    // below range flushes to zero with underflow set.
    let o = sse::g7_cvtss2sd(0x7FC1_2345);
    assert!(is_qnan64(o.bits), "single qNaN becomes double qNaN");
    assert_eq!(o.mxcsr & MXCSR_IE, 0);
    let o = sse::g7_cvtss2sd(0x7F80_0001);
    assert!(is_qnan64(o.bits), "single sNaN becomes double qNaN");
    assert_eq!(o.mxcsr & MXCSR_IE, MXCSR_IE, "sNaN raises");
    let o = sse::g7_cvtss2sd(0x7F80_0000);
    assert_eq!(o.bits, 0x7FF0_0000_0000_0000, "+inf converts exactly");
    let o = sse::g7_cvtss2sd(0x0000_0001);
    assert_eq!(o.bits, 0x36A0_0000_0000_0000, "2^-149 converts exactly");
    let o = sse::g7_cvtsd2ss(0x7FF8_0000_0001_1111);
    assert!(is_qnan32(o.bits), "double qNaN becomes single qNaN");
    assert_eq!(o.mxcsr & MXCSR_IE, 0);
    let o = sse::g7_cvtsd2ss(0x3FD5_5555_5555_5555);
    assert_eq!(o.bits, 0x3EAA_AAAB, "double 1/3 rounds to single 1/3");
    assert_eq!(o.mxcsr & MXCSR_PE, MXCSR_PE, "rounding sets inexact");
    let o = sse::g7_cvtsd2ss(0x54B2_49AD_2594_C37D);
    assert_eq!(o.bits, 0x7F80_0000, "1e100 overflows single to +inf");
    assert_ne!(o.mxcsr & 0x0008, 0, "overflow flag set");
    let o = sse::g7_cvtsd2ss(0x0000_0000_0000_0001);
    assert_eq!(o.bits, 0x0000_0000, "2^-1074 is zero as a single");
    assert_ne!(o.mxcsr & 0x0010, 0, "underflow flag set");
}

#[test]
fn g7_cvtsi_int_to_float() {
    // KNOWN-EMULATOR-DIFF (g7-cvtsi-PE): CVTSI2SS never sets PE on rounding; values correct.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff("g7-cvtsi-PE", "CVTSI2SS rounding PE never set");
    // Rule: integer-to-float conversions are exact when the value fits and
    // round to nearest with inexact set when it does not.
    let o = sse::g7_cvtsi2ss(1);
    assert_eq!(o.bits, 0x3F80_0000);
    assert_eq!(o.mxcsr & MXCSR_PE, 0);
    let o = sse::g7_cvtsi2ss(16_777_217);
    assert_eq!(o.bits, 0x4B80_0000, "2^24+1 rounds to 2^24");
    assert_eq!(o.mxcsr & MXCSR_PE, MXCSR_PE);
    let o = sse::g7_cvtsi2sd(1);
    assert_eq!(o.bits, 0x3FF0_0000_0000_0000);
    let o = sse::g7_cvtsi2sd(16_777_217);
    assert_eq!(o.bits, 0x4170_0000_1000_0000, "exact as a double");
    assert_eq!(o.mxcsr & MXCSR_PE, 0);
}

#[test]
fn g7_ftz_denormal_pair() {
    // Rule: with flush-to-zero off (the default) scalar SSE preserves
    // denormals; with it on, a tiny result flushes to zero.
    let o = sse::g7_addss_mxcsr(MXCSR_DEFAULT, 0x0000_0001, 0x0000_0001);
    assert_eq!(o.bits, 0x0000_0002, "denormals add exactly with FTZ off");
    let o = sse::g7_addss_mxcsr(MXCSR_DEFAULT | 0x8000, 0x0000_0001, 0x0000_0001);
    assert_eq!(o.bits, 0x0000_0000, "tiny result flushes with FTZ on");
}

#[test]
fn info_emulation_details() {
    // Not a behaviour assertion: records where the suite ran.
    let d = emulation_details();
    eprintln!("arch={} os={} emulated={}", d.arch, d.os, d.emulated);
    assert_eq!(is_emulated(), d.emulated);
}
