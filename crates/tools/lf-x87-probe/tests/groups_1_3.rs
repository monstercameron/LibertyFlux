//! Documented-behaviour tests, groups 1-3. Each test states the Intel
//! manual's rule in words and asserts the raw bits a real x86 processor
//! produces. x86 only; on other targets this file compiles to nothing.
#![cfg(target_arch = "x86")]

use lf_x87_probe::sse;
use lf_x87_probe::x87;
use lf_x87_probe::x87cmp;
use lf_x87_probe::{
    MXCSR_DEFAULT, MXCSR_IE, MXCSR_PE, MXCSR_RC_SHIFT, PC_24, PC_53, PC_64, RC_CHOP, RC_DOWN,
    RC_NEAR, RC_UP, SW_IE, SW_PE,
};

fn is_qnan32(v: u32) -> bool {
    v & 0x7F80_0000 == 0x7F80_0000 && v & 0x007F_FFFF != 0 && v & 0x0040_0000 != 0
}

fn is_qnan64(v: u64) -> bool {
    v & 0x7FF0_0000_0000_0000 == 0x7FF0_0000_0000_0000
        && v & 0x000F_FFFF_FFFF_FFFF != 0
        && v & 0x0008_0000_0000_0000 != 0
}

// ---------------------------------------------------------------------------
// Group 1.
// ---------------------------------------------------------------------------

#[test]
fn g1_f32_snan_quieted_mem() {
    // KNOWN-EMULATOR-DIFF (g1-f32-snan-IE): fld m32 quiets the value but leaves IE clear.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff(
        "g1-f32-snan-IE",
        "fld m32 quiets f32 sNaN but IE stays clear",
    );
    // Rule: loading a signalling NaN raises invalid and quiets it (sets the
    // quiet bit), preserving sign and payload; with the exception masked the
    // stored value is the quieted NaN.
    for (input, expected) in [
        (0x7F80_0001u32, 0x7FC0_0001u32),
        (0x7F80_0123u32, 0x7FC0_0123u32),
        (0xFF80_0001u32, 0xFFC0_0001u32),
    ] {
        let o = x87::g1_f32_mem_roundtrip(input);
        assert_eq!(o.bits, expected, "input {input:#010x}");
        assert_eq!(o.status & SW_IE, SW_IE, "input {input:#010x}: IE flag");
    }
}

#[test]
fn g1_f32_plain_values_mem() {
    // Rule: quiet NaNs pass through loads and stores unchanged without an
    // exception; denormals, infinities and zeros round-trip exactly.
    for v in [
        0x7FC0_0000u32,
        0x7FC1_2345,
        0xFFC0_0000,
        0x0000_0001,
        0x007F_FFFF,
        0x8000_0001,
        0x7F80_0000,
        0xFF80_0000,
        0x8000_0000,
        0x0000_0000,
        0x3F80_0000,
    ] {
        let o = x87::g1_f32_mem_roundtrip(v);
        assert_eq!(o.bits, v, "input {v:#010x}");
        assert_eq!(o.status & SW_IE, 0, "input {v:#010x}: no IE");
    }
}

#[test]
fn g1_f64_snan_quieted_mem() {
    // KNOWN-EMULATOR-DIFF (g1-f64-snan-quiet): fld m64 leaves f64 sNaN unquieted, IE clear.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff(
        "g1-f64-snan-quiet",
        "fld m64 does not quiet f64 sNaN, IE clear",
    );
    // Rule: as for 32-bit: a 64-bit signalling NaN is quieted on load with
    // the invalid flag set, sign and payload preserved.
    for (input, expected) in [
        (0x7FF0_0000_0000_0001u64, 0x7FF8_0000_0000_0001u64),
        (0x7FF0_0000_0000_1234u64, 0x7FF8_0000_0000_1234u64),
        (0xFFF0_0000_0000_0001u64, 0xFFF8_0000_0000_0001u64),
    ] {
        let o = x87::g1_f64_mem_roundtrip(input);
        assert_eq!(o.bits, expected, "input {input:#018x}");
        assert_eq!(o.status & SW_IE, SW_IE, "input {input:#018x}: IE flag");
    }
}

#[test]
fn g1_f64_plain_values_mem() {
    // Rule: 64-bit quiet NaNs, denormals, infinities and zeros round-trip.
    for v in [
        0x7FF8_0000_0000_0000u64,
        0x7FF8_0000_0000_1234,
        0xFFC0_0000_0000_0000,
        0x0000_0000_0000_0001,
        0x000F_FFFF_FFFF_FFFF,
        0x8000_0000_0000_0001,
        0x7FF0_0000_0000_0000,
        0xFFF0_0000_0000_0000,
        0x8000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x3FF0_0000_0000_0000,
    ] {
        let o = x87::g1_f64_mem_roundtrip(v);
        assert_eq!(o.bits, v, "input {v:#018x}");
        assert_eq!(o.status & SW_IE, 0, "input {v:#018x}: no IE");
    }
}

#[test]
fn g1_f80_values_mem() {
    // KNOWN-EMULATOR-DIFF (g1-f80-snan-quiet): fld m80 leaves sNaN unquieted, IE clear.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff("g1-f80-snan-quiet", "fld m80 does not quiet sNaN, IE clear");
    // Rule: an 80-bit signalling NaN (exponent all ones, integer bit set,
    // quiet bit, bit 62, clear) is quieted on load with invalid raised;
    // quiet NaNs, infinities and zeros pass through unchanged.
    let o = x87::g1_f80_mem_roundtrip(0x8000_0000_0000_0001, 0x7FFF);
    assert_eq!((o.mant, o.exp), (0xC000_0000_0000_0001, 0x7FFF));
    assert_eq!(o.status & SW_IE, SW_IE, "sNaN sets IE");
    for (m, e) in [
        (0xC000_0000_0000_0000u64, 0x7FFFu16),
        (0xC000_0000_0000_0000, 0xFFFF),
        (0x8000_0000_0000_0000, 0x7FFF),
        (0x8000_0000_0000_0000, 0xFFFF),
        (0x0000_0000_0000_0000, 0x8000),
        (0x8000_0000_0000_0000, 0x3FFF),
    ] {
        let o = x87::g1_f80_mem_roundtrip(m, e);
        assert_eq!((o.mant, o.exp), (m, e), "input {m:#018x}:{e:#06x}");
        assert_eq!(o.status & SW_IE, 0, "no IE");
    }
    // 80-bit denormal round-trips (denormal flag recorded, not asserted).
    let o = x87::g1_f80_mem_roundtrip(0x0000_0000_0000_0001, 0x0000);
    assert_eq!((o.mant, o.exp), (0x0000_0000_0000_0001, 0x0000));
}

#[test]
fn g1_f32_return_quieting() {
    // KNOWN-EMULATOR-DIFF (g1-f32-ret-snan-IE): ST(0) return arrives quieted but IE clear.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff(
        "g1-f32-ret-snan-IE",
        "f32 sNaN via ST(0) quieted but IE clear",
    );
    // Rule: a 32-bit function returns a float in ST(0), loaded there by the
    // callee, so a signalling NaN is quieted exactly as by `fld`.
    let o = x87::g1_f32_return_roundtrip(0x7F80_0001);
    assert_eq!(o.bits, 0x7FC0_0001);
    assert_eq!(o.status & SW_IE, SW_IE, "sNaN return sets IE");
    for (v, expected) in [
        (0x7FC1_2345u32, 0x7FC1_2345u32),
        (0x0000_0001, 0x0000_0001),
        (0x7F80_0000, 0x7F80_0000),
        (0x8000_0000, 0x8000_0000),
    ] {
        let o = x87::g1_f32_return_roundtrip(v);
        assert_eq!(o.bits, expected, "input {v:#010x}");
        assert_eq!(o.status & SW_IE, 0, "no IE");
    }
}

#[test]
fn g1_f64_return_quieting() {
    // KNOWN-EMULATOR-DIFF (g1-f64-ret-snan-quiet): ST(0) return arrives unquieted, IE clear.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff(
        "g1-f64-ret-snan-quiet",
        "f64 sNaN via ST(0) unquieted, IE clear",
    );
    // Rule: as for float: a double returned in ST(0) was loaded there, so a
    // signalling NaN arrives quieted with the invalid flag set.
    let o = x87::g1_f64_return_roundtrip(0x7FF0_0000_0000_0001);
    assert_eq!(o.bits, 0x7FF8_0000_0000_0001);
    assert_eq!(o.status & SW_IE, SW_IE, "sNaN return sets IE");
    for (v, expected) in [
        (0x7FF8_0000_0000_1234u64, 0x7FF8_0000_0000_1234u64),
        (0x0000_0000_0000_0001, 0x0000_0000_0000_0001),
        (0x7FF0_0000_0000_0000, 0x7FF0_0000_0000_0000),
        (0x8000_0000_0000_0000, 0x8000_0000_0000_0000),
    ] {
        let o = x87::g1_f64_return_roundtrip(v);
        assert_eq!(o.bits, expected, "input {v:#018x}");
        assert_eq!(o.status & SW_IE, 0, "no IE");
    }
}

// ---------------------------------------------------------------------------
// Group 2. Expected 80-bit values are the correctly rounded results from
// a decimal oracle kept outside the repository (120 digits, half to even).
// ---------------------------------------------------------------------------

const F64_ONE: u64 = 0x3FF0_0000_0000_0000;
const F64_P2M25: u64 = 0x3E60_0000_0000_0000;
const F64_P2M54: u64 = 0x3C90_0000_0000_0000;
const F64_1P2M12: u64 = 0x3FF0_0100_0000_0000;
const F64_1P2M26: u64 = 0x3FF0_0000_0400_0000;
const F64_1P2M27: u64 = 0x3FF0_0000_0200_0000;
const F64_THREE: u64 = 0x4008_0000_0000_0000;
const F64_TWO: u64 = 0x4000_0000_0000_0000;
const X80_ONE: (u64, u16) = (0x8000_0000_0000_0000, 0x3FFF);

#[test]
fn g2_add_rounds_to_pc() {
    // KNOWN-EMULATOR-DIFF (g2-add-PC): FADD always computes at 53 bits, PE never set.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff("g2-add-PC", "FADD ignores PC (53-bit) and PE");
    // Rule: x87 arithmetic rounds the infinitely precise result to the
    // mantissa width selected by the precision field, round to nearest.
    // 1+2^-25 is exact at 53 and 64 bits but rounds to 1 at 24 bits;
    // 1+2^-54 is exact only at 64 bits.
    let o = x87::g2_fadd(PC_24, F64_ONE, F64_P2M25);
    assert_eq!((o.mant, o.exp), X80_ONE, "24 bits rounds 1+2^-25 to 1");
    assert_eq!(o.status & SW_PE, SW_PE, "inexact at 24 bits");
    for pc in [PC_53, PC_64] {
        let o = x87::g2_fadd(pc, F64_ONE, F64_P2M25);
        assert_eq!(
            (o.mant, o.exp),
            (0x8000_0040_0000_0000, 0x3FFF),
            "pc {pc:#06x}"
        );
        assert_eq!(o.status & SW_PE, 0, "exact above 24 bits");
    }
    for pc in [PC_24, PC_53] {
        let o = x87::g2_fadd(pc, F64_ONE, F64_P2M54);
        assert_eq!((o.mant, o.exp), X80_ONE, "pc {pc:#06x} rounds to 1");
        assert_eq!(o.status & SW_PE, SW_PE, "inexact below 64 bits");
    }
    let o = x87::g2_fadd(PC_64, F64_ONE, F64_P2M54);
    assert_eq!((o.mant, o.exp), (0x8000_0000_0000_0200, 0x3FFF));
    assert_eq!(o.status & SW_PE, 0, "exact at 64 bits");
}

#[test]
fn g2_sub_rounds_to_pc() {
    // KNOWN-EMULATOR-DIFF (g2-sub-PC): FSUB always computes at 53 bits, PE never set.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff("g2-sub-PC", "FSUB ignores PC (53-bit) and PE");
    // Rule: as for add. 1-2^-25 is exact at 53 and 64 bits, 1 at 24 bits;
    // 1-2^-54 is exact only at 64 bits.
    let o = x87::g2_fsub(PC_24, F64_ONE, F64_P2M25);
    assert_eq!((o.mant, o.exp), X80_ONE);
    assert_eq!(o.status & SW_PE, SW_PE);
    for pc in [PC_53, PC_64] {
        let o = x87::g2_fsub(pc, F64_ONE, F64_P2M25);
        assert_eq!(
            (o.mant, o.exp),
            (0xFFFF_FF80_0000_0000, 0x3FFE),
            "pc {pc:#06x}"
        );
        assert_eq!(o.status & SW_PE, 0);
    }
    for pc in [PC_24, PC_53] {
        let o = x87::g2_fsub(pc, F64_ONE, F64_P2M54);
        assert_eq!((o.mant, o.exp), X80_ONE, "pc {pc:#06x}");
        assert_eq!(o.status & SW_PE, SW_PE);
    }
    let o = x87::g2_fsub(PC_64, F64_ONE, F64_P2M54);
    assert_eq!((o.mant, o.exp), (0xFFFF_FFFF_FFFF_FC00, 0x3FFE));
    assert_eq!(o.status & SW_PE, 0);
}

#[test]
fn g2_mul_rounds_to_pc() {
    // KNOWN-EMULATOR-DIFF (g2-mul-PC64-PE): FMUL caps at 53 bits, PE never set (PC=24 honored).
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff("g2-mul-PC64-PE", "FMUL caps at 53 bits, PE never set");
    // Rule: as for add. (1+2^-12)^2 needs 25 bits, so 24-bit precision
    // drops the last term; (1+2^-26)(1+2^-27) needs 54 bits, so 53-bit
    // precision drops it by ties-to-even and 24-bit rounds to 1.
    let o = x87::g2_fmul(PC_24, F64_1P2M12, F64_1P2M12);
    assert_eq!((o.mant, o.exp), (0x8010_0000_0000_0000, 0x3FFF));
    assert_eq!(o.status & SW_PE, SW_PE);
    for pc in [PC_53, PC_64] {
        let o = x87::g2_fmul(pc, F64_1P2M12, F64_1P2M12);
        assert_eq!(
            (o.mant, o.exp),
            (0x8010_0080_0000_0000, 0x3FFF),
            "pc {pc:#06x}"
        );
        assert_eq!(o.status & SW_PE, 0);
    }
    let o = x87::g2_fmul(PC_24, F64_1P2M26, F64_1P2M27);
    assert_eq!((o.mant, o.exp), X80_ONE);
    assert_eq!(o.status & SW_PE, SW_PE);
    let o = x87::g2_fmul(PC_53, F64_1P2M26, F64_1P2M27);
    assert_eq!((o.mant, o.exp), (0x8000_0030_0000_0000, 0x3FFF));
    assert_eq!(o.status & SW_PE, SW_PE, "tie dropped at 53 bits");
    let o = x87::g2_fmul(PC_64, F64_1P2M26, F64_1P2M27);
    assert_eq!((o.mant, o.exp), (0x8000_0030_0000_0400, 0x3FFF));
    assert_eq!(o.status & SW_PE, 0);
}

#[test]
fn g2_div_sqrt_round_to_pc() {
    // KNOWN-EMULATOR-DIFF (g2-div-sqrt-PC): FDIV/FSQRT always compute at 53 bits, PE never set.
    // Assertion unchanged: must pass on real x86.
    lf_x87_probe::note_emulator_diff("g2-div-sqrt-PC", "FDIV/FSQRT ignore PC (53-bit) and PE");
    // Rule: divide and square root are correctly rounded at every precision
    // setting, so 1/3 and sqrt(2) each take three distinct values.
    for (pc, mant, exp) in [
        (PC_24, 0xAAAA_AB00_0000_0000u64, 0x3FFDu16),
        (PC_53, 0xAAAA_AAAA_AAAA_A800, 0x3FFD),
        (PC_64, 0xAAAA_AAAA_AAAA_AAAB, 0x3FFD),
    ] {
        let o = x87::g2_fdiv(pc, F64_ONE, F64_THREE);
        assert_eq!((o.mant, o.exp), (mant, exp), "div pc {pc:#06x}");
        assert_eq!(o.status & SW_PE, SW_PE, "1/3 always inexact");
    }
    for (pc, mant, exp) in [
        (PC_24, 0xB504_F300_0000_0000u64, 0x3FFFu16),
        (PC_53, 0xB504_F333_F9DE_6800, 0x3FFF),
        (PC_64, 0xB504_F333_F9DE_6484, 0x3FFF),
    ] {
        let o = x87::g2_fsqrt(pc, F64_TWO);
        assert_eq!((o.mant, o.exp), (mant, exp), "sqrt pc {pc:#06x}");
        assert_eq!(o.status & SW_PE, SW_PE, "sqrt(2) always inexact");
    }
}

// ---------------------------------------------------------------------------
// Group 3 (integer conversions).
// ---------------------------------------------------------------------------

const F64_P2_5: u64 = 0x4004_0000_0000_0000;
const F64_N2_5: u64 = 0xC004_0000_0000_0000;
const F64_P2_7: u64 = 0x4005_9999_9999_999A;
const F64_N2_7: u64 = 0xC005_9999_9999_999A;

#[test]
fn g3_fistp32_rounding_modes() {
    // Rule: FISTP rounds to an integer using the rounding field: nearest
    // breaks ties to even, down goes toward -inf, up toward +inf, chop
    // toward zero.
    for (rc, p2_5, n2_5, p2_7, n2_7) in [
        (RC_NEAR, 2u32, 0xFFFF_FFFEu32, 3u32, 0xFFFF_FFFDu32),
        (RC_DOWN, 2, 0xFFFF_FFFD, 2, 0xFFFF_FFFD),
        (RC_UP, 3, 0xFFFF_FFFE, 3, 0xFFFF_FFFE),
        (RC_CHOP, 2, 0xFFFF_FFFE, 2, 0xFFFF_FFFE),
    ] {
        for (v, expected) in [
            (F64_P2_5, p2_5),
            (F64_N2_5, n2_5),
            (F64_P2_7, p2_7),
            (F64_N2_7, n2_7),
        ] {
            let o = x87cmp::g3_fistp_i32(rc, v);
            assert_eq!(o.val, expected, "rc {rc:#06x} input {v:#018x}");
            assert_eq!(o.status & SW_IE, 0, "no IE");
        }
    }
}

#[test]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
// The table holds small signed values (±2, ±3); the casts below are
// intentional two's-complement reinterprets into the store widths.
fn g3_fistp16_64_rounding_modes() {
    // Rule: as for 32-bit, for the 16-bit and 64-bit integer stores.
    for (rc, p, n) in [
        (RC_NEAR, 2i64, -2i64),
        (RC_DOWN, 2, -3),
        (RC_UP, 3, -2),
        (RC_CHOP, 2, -2),
    ] {
        let o = x87cmp::g3_fistp_i16(rc, F64_P2_5);
        assert_eq!(o.val, p as u16, "i16 rc {rc:#06x} +2.5");
        let o = x87cmp::g3_fistp_i16(rc, F64_N2_5);
        assert_eq!(o.val, n as u16, "i16 rc {rc:#06x} -2.5");
        let o = x87cmp::g3_fistp_i64(rc, F64_P2_5);
        assert_eq!(o.val, p as u64, "i64 rc {rc:#06x} +2.5");
        let o = x87cmp::g3_fistp_i64(rc, F64_N2_5);
        assert_eq!(o.val, n as u64, "i64 rc {rc:#06x} -2.5");
    }
}

#[test]
fn g3_fistp_integer_indefinite() {
    // Rule: an out-of-range, infinite or NaN input to FISTP stores the
    // integer indefinite value (a 1 followed by zeros) and raises invalid.
    for (v, tag) in [
        (0x4415_AF1D_78B5_8C40u64, "1e20"),
        (0x7FF8_0000_0000_0000u64, "qNaN"),
        (0x7FF0_0000_0000_0000u64, "+inf"),
    ] {
        let o = x87cmp::g3_fistp_i32(RC_NEAR, v);
        assert_eq!(o.val, 0x8000_0000, "i32 {tag}");
        assert_eq!(o.status & SW_IE, SW_IE, "i32 {tag} IE");
    }
    for (v, tag) in [
        (0x4202_A05F_2000_0000u64, "1e10"),
        (0x7FF8_0000_0000_0000u64, "qNaN"),
    ] {
        let o = x87cmp::g3_fistp_i16(RC_NEAR, v);
        assert_eq!(o.val, 0x8000, "i16 {tag}");
        assert_eq!(o.status & SW_IE, SW_IE, "i16 {tag} IE");
    }
    for (v, tag) in [
        (0x4415_AF1D_78B5_8C40u64, "1e20"),
        (0x7FF8_0000_0000_0000u64, "qNaN"),
    ] {
        let o = x87cmp::g3_fistp_i64(RC_NEAR, v);
        assert_eq!(o.val, 0x8000_0000_0000_0000, "i64 {tag}");
        assert_eq!(o.status & SW_IE, SW_IE, "i64 {tag} IE");
    }
}

#[test]
fn g3_fist_matches_fistp() {
    // Rule: FIST (store without pop) rounds exactly like FISTP.
    let o = x87cmp::g3_fist_i32(RC_NEAR, F64_P2_5);
    assert_eq!(o.val, 2);
    let o = x87cmp::g3_fist_i32(RC_NEAR, F64_N2_5);
    assert_eq!(o.val, 0xFFFF_FFFE);
}

#[test]
fn g3_fisttp_truncates() {
    // Rule: FISTTP always truncates toward zero, ignoring the rounding
    // field; invalid inputs give the indefinite value with invalid raised.
    for (v, expected32, expected64, ie) in [
        (F64_P2_5, 2u32, 2u64, false),
        (F64_N2_5, 0xFFFF_FFFEu32, 0xFFFF_FFFF_FFFF_FFFEu64, false),
        (
            0x4415_AF1D_78B5_8C40u64,
            0x8000_0000u32,
            0x8000_0000_0000_0000u64,
            true,
        ),
        (
            0x7FF8_0000_0000_0000u64,
            0x8000_0000u32,
            0x8000_0000_0000_0000u64,
            true,
        ),
    ] {
        let o = x87cmp::g3_fisttp_i32(v).expect("SSE3 is present on any modern x86");
        assert_eq!(o.val, expected32, "i32 input {v:#018x}");
        assert_eq!(o.status & SW_IE != 0, ie, "i32 IE");
        let o = x87cmp::g3_fisttp_i64(v).expect("SSE3 is present on any modern x86");
        assert_eq!(o.val, expected64, "i64 input {v:#018x}");
        assert_eq!(o.status & SW_IE != 0, ie, "i64 IE");
    }
}

#[test]
fn g3_cvt_rounding_modes() {
    // Rule: CVTSS2SI and CVTSD2SI round using the MXCSR rounding field with
    // the same four meanings as the x87 field.
    for (rc, p, n) in [
        (RC_NEAR, 2i32, -2i32),
        (RC_DOWN, 2, -3),
        (RC_UP, 3, -2),
        (RC_CHOP, 2, -2),
    ] {
        let mx = MXCSR_DEFAULT | ((u32::from(rc) >> 10) << MXCSR_RC_SHIFT);
        let (v, flags) = sse::g3_cvtss2si(mx, 0x4020_0000);
        assert_eq!(v, p, "ss rc {rc:#06x} +2.5");
        assert_eq!(flags & MXCSR_IE, 0);
        let (v, _) = sse::g3_cvtss2si(mx, 0xC020_0000);
        assert_eq!(v, n, "ss rc {rc:#06x} -2.5");
        let (v, _) = sse::g3_cvtsd2si(mx, F64_P2_5);
        assert_eq!(v, p, "sd rc {rc:#06x} +2.5");
        let (v, _) = sse::g3_cvtsd2si(mx, F64_N2_5);
        assert_eq!(v, n, "sd rc {rc:#06x} -2.5");
    }
}

#[test]
fn g3_cvtt_truncates() {
    // Rule: the truncating vector conversions round toward zero; invalid
    // inputs give 0x80000000 with the MXCSR invalid flag set.
    for (v, expected, ie) in [
        (0x4020_0000u32, 2i32, false),
        (0xC020_0000u32, -2i32, false),
        (0x60AD_78ECu32, i32::MIN, true),
        (0x7FC0_0000u32, i32::MIN, true),
    ] {
        let (val, flags) = sse::g3_cvttss2si(v);
        assert_eq!(val, expected, "ss input {v:#010x}");
        assert_eq!(flags & MXCSR_IE != 0, ie, "ss IE");
    }
    for (v, expected, ie) in [
        (F64_P2_5, 2i32, false),
        (F64_N2_5, -2i32, false),
        (0x4415_AF1D_78B5_8C40u64, i32::MIN, true),
        (0x7FF8_0000_0000_0000u64, i32::MIN, true),
    ] {
        let (val, flags) = sse::g3_cvttsd2si(v);
        assert_eq!(val, expected, "sd input {v:#018x}");
        assert_eq!(flags & MXCSR_IE != 0, ie, "sd IE");
    }
}

#[test]
fn g3_helpers_covered() {
    // Keeps the shared NaN predicates exercised so they cannot rot.
    assert!(is_qnan32(0x7FC0_0001));
    assert!(is_qnan64(0x7FF8_0000_0000_0001));
    assert!(!is_qnan32(0x7F80_0001));
    assert!(!is_qnan64(0x7FF0_0000_0000_0001));
    assert_eq!(MXCSR_PE, 0x0020);
}
