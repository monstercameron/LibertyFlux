// original: 0x008B1270 biquad coefficient update
/// Recalculates one coefficient row for the active biquad filter mode.
///
/// The method exits when bypass is active or the filter mode is outside the
/// supported range. Otherwise it reads the current sample rate, computes a
/// clamped frequency and normalized angular rate, then selects one of six
/// coefficient families for modes 0 through 10. The cosine, sine, tangent,
/// and x87 helper calls are explicit dependencies; this rewrite preserves
/// their input order and applies their returned values to the selected row.
/// The contract exercises every supported mode and both stage settings.

use core::arch::x86::{_mm_add_ss, _mm_cvtss_f32, _mm_div_ss, _mm_mul_ss, _mm_set_ss, _mm_sub_ss};

const SAMPLE_RATE_MANAGER_VA: u32 = 0x0115a448;
const SAMPLE_RATE_OFFSET: u32 = 0x80;
const MODE_OFFSET: u32 = 0xbc;
const BYPASS_OFFSET: u32 = 0xb8;
const STAGE_OFFSET: u32 = 0xc3;
const INDEX_OFFSET: u32 = 8;
const GAIN_OFFSET: u32 = 0xa8;
const Q_OFFSET: u32 = 0xac;
const LIMIT_OFFSET: u32 = 0xb0;
const TAU_VA: u32 = 0x00fe8aec;
const PI_VA: u32 = 0x00fe8aa0;
const TWO_VA: u32 = 0x00fe8a24;
const HALF_VA: u32 = 0x00fe8830;
const ONE_VA: u32 = 0x00fe88e8;
const NEG_ONE_VA: u32 = 0x00fe8d94;
const NEG_TWO_VA: u32 = 0x00fe8db0;
const SQRT_TWO_VA: u32 = 0x00e7cd38;
const COS_ID: u32 = 0;
const SIN_ID: u32 = 1;
const TAN_X87_ID: u32 = 2;
const TAN_SSE_ID: u32 = 3;
const SIGN_BIT: u32 = 0x80000000;

#[target_feature(enable = "sse")]
unsafe fn fadd(lhs: f32, rhs: f32) -> f32 {
    let left = _mm_set_ss(core::hint::black_box(lhs));
    let right = _mm_set_ss(core::hint::black_box(rhs));
    _mm_cvtss_f32(_mm_add_ss(left, right))
}

#[target_feature(enable = "sse")]
unsafe fn fsub(lhs: f32, rhs: f32) -> f32 {
    let left = _mm_set_ss(core::hint::black_box(lhs));
    let right = _mm_set_ss(core::hint::black_box(rhs));
    _mm_cvtss_f32(_mm_sub_ss(left, right))
}

#[target_feature(enable = "sse")]
unsafe fn fmul(lhs: f32, rhs: f32) -> f32 {
    let left = _mm_set_ss(core::hint::black_box(lhs));
    let right = _mm_set_ss(core::hint::black_box(rhs));
    _mm_cvtss_f32(_mm_mul_ss(left, right))
}

#[target_feature(enable = "sse")]
unsafe fn fdiv(lhs: f32, rhs: f32) -> f32 {
    let left = _mm_set_ss(core::hint::black_box(lhs));
    let right = _mm_set_ss(core::hint::black_box(rhs));
    _mm_cvtss_f32(_mm_div_ss(left, right))
}

#[inline(always)]
fn fneg(value: f32) -> f32 {
    f32::from_bits(value.to_bits() ^ SIGN_BIT)
}

#[inline(always)]
fn global_f32(file_va: u32) -> f32 {
    unsafe { f32::from_bits(lf_checker_rt::global::<u32>(file_va).read_unaligned()) }
}

#[inline(always)]
fn global_u32(file_va: u32) -> u32 {
    unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
}

#[inline(always)]
fn call_f32_xmm0(id: u32, value: f32) -> f32 {
    let answer: u32 = lf_checker_rt::callee_cdecl!(id, u32, value.to_bits());
    f32::from_bits(answer)
}

#[inline(always)]
fn call_f64_st0(id: u32, value: f64) -> f64 {
    let bits = value.to_bits();
    lf_checker_rt::callee_cdecl!(id, f64, bits as u32, (bits >> 32) as u32)
}

#[inline(always)]
unsafe fn read32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn read8(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn write32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

unsafe fn write_row(
    this: u32,
    index: u32,
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    mutant: bool,
    mode: u32,
) {
    unsafe {
        let row = this + index.wrapping_mul(4);
        let b0_bits = b0.to_bits() ^ ((mutant && mode == 0) as u32);
        write32(row + 0x0c, b0_bits);
        write32(row + 0x18, b1.to_bits());
        write32(row + 0x24, b2.to_bits());
        write32(row + 0x30, a1.to_bits());
        write32(row + 0x3c, a2.to_bits());
    }
}

unsafe fn update_coefficients(this: u32, mutant: bool) -> u32 {
    unsafe {
        if read8(this + BYPASS_OFFSET) != 0 {
            return 0;
        }

        let mode = read32(this + MODE_OFFSET);
        if mode > 10 {
            return 0;
        }

        let manager = global_u32(SAMPLE_RATE_MANAGER_VA);
        let sample_rate = read32(manager + SAMPLE_RATE_OFFSET) as f32;
        let gain = f32::from_bits(read32(this + GAIN_OFFSET));
        let limit = f32::from_bits(read32(this + LIMIT_OFFSET));
        let two_gain = fmul(gain, global_f32(TWO_VA));
        let cutoff = if two_gain > limit { limit } else { two_gain };
        let scaled_rate = fmul(gain, global_f32(TAU_VA));
        let inverse_rate = fdiv(1.0, sample_rate);
        let omega = fmul(scaled_rate, inverse_rate);
        let index = read32(this + INDEX_OFFSET);

        let one = global_f32(ONE_VA);
        let half = global_f32(HALF_VA);
        let two = global_f32(TWO_VA);
        let minus_one = global_f32(NEG_ONE_VA);
        let minus_two = global_f32(NEG_TWO_VA);
        let quality = f32::from_bits(read32(this + Q_OFFSET));

        match mode {
            0 | 1 => {
                let stage_scale = if read8(this + STAGE_OFFSET) != 0 { half } else { one };
                let cosine = call_f32_xmm0(COS_ID, omega);
                let scaled_quality = fmul(quality, stage_scale);
                let tangent_input = fdiv(half, scaled_quality) as f64;
                let tangent = call_f64_st0(TAN_X87_ID, tangent_input) as f32;
                let sine = call_f32_xmm0(SIN_ID, omega);
                let alpha = fmul(tangent, sine);
                let denominator = fadd(alpha, one);
                let inverse = fdiv(one, denominator);
                let one_minus_cosine = fsub(one, cosine);
                let half_delta = fmul(one_minus_cosine, half);
                let b0 = fmul(inverse, half_delta);
                let b1 = fmul(inverse, one_minus_cosine);
                let a1 = fneg(fmul(fmul(cosine, two), inverse));
                let a2 = fmul(fsub(one, alpha), inverse);
                write_row(this, index, b0, b1, b0, a1, a2, mutant, mode);
            }
            2 | 3 => {
                let stage_scale = if read8(this + STAGE_OFFSET) != 0 { half } else { one };
                let cosine = call_f32_xmm0(COS_ID, omega);
                let scaled_quality = fmul(quality, stage_scale);
                let tangent_input = fdiv(half, scaled_quality) as f64;
                let tangent = call_f64_st0(TAN_X87_ID, tangent_input) as f32;
                let sine = call_f32_xmm0(SIN_ID, omega);
                let alpha = fmul(tangent, sine);
                let inverse = fdiv(one, fadd(alpha, one));
                let one_plus_cosine = fadd(cosine, one);
                let half_sum = fmul(one_plus_cosine, half);
                let b0 = fmul(inverse, half_sum);
                let b1 = fmul(fsub(minus_one, cosine), inverse);
                let a1 = fmul(fmul(cosine, minus_two), inverse);
                let a2 = fmul(fsub(one, alpha), inverse);
                write_row(this, index, b0, b1, b0, a1, a2, mutant, mode);
            }
            4 | 5 => {
                let tangent_input = fmul(fmul(inverse_rate, cutoff), global_f32(PI_VA));
                let tangent = call_f32_xmm0(TAN_SSE_ID, tangent_input);
                let inverse_tangent = fdiv(one, tangent);
                let inverse = fdiv(one, fadd(inverse_tangent, one));
                let b0 = inverse;
                let b1 = 0.0;
                let b2 = fneg(b0);
                let cosine = call_f32_xmm0(COS_ID, omega);
                let b0_times_inverse_tangent = fmul(b0, inverse_tangent);
                let a1 = fneg(fmul(fmul(cosine, two), b0_times_inverse_tangent));
                let a2 = fmul(fsub(inverse_tangent, one), b0);
                write_row(this, index, b0, b1, b2, a1, a2, mutant, mode);
            }
            6 | 7 => {
                let tangent_input = fmul(fmul(inverse_rate, cutoff), global_f32(PI_VA));
                let tangent = call_f32_xmm0(TAN_SSE_ID, tangent_input);
                let inverse = fdiv(one, fadd(tangent, one));
                let b0 = inverse;
                let cosine = call_f32_xmm0(COS_ID, omega);
                let a1 = fmul(fmul(cosine, minus_two), inverse);
                let a2 = fmul(fsub(one, tangent), inverse);
                write_row(this, index, b0, a1, b0, a1, a2, mutant, mode);
            }
            8 | 9 | 10 => {
                let sine = call_f32_xmm0(SIN_ID, omega);
                let ratio_denominator = fmul(fdiv(gain, cutoff), two);
                let ratio = fdiv(sine, ratio_denominator);
                let cosine = call_f32_xmm0(COS_ID, omega);
                let effective_quality = if mode == 10 {
                    fmul(quality, global_f32(SQRT_TWO_VA))
                } else {
                    quality
                };
                let ratio_over_quality = fdiv(ratio, effective_quality);
                let quality_times_ratio = fmul(effective_quality, ratio);
                let inverse = fdiv(one, fadd(ratio_over_quality, one));
                let b0 = fmul(fadd(quality_times_ratio, one), inverse);
                let b1 = fmul(fmul(cosine, minus_two), inverse);
                let b2 = fmul(fsub(one, quality_times_ratio), inverse);
                let a2 = fmul(fsub(one, ratio_over_quality), inverse);
                write_row(this, index, b0, b1, b2, b1, a2, mutant, mode);
            }
            _ => unreachable!(),
        }
        0
    }
}

lf_checker_rt::export!(thiscall, rw_008B1270(this: u32) -> u32 {
    unsafe { update_coefficients(this, false) }
});

lf_checker_rt::export!(thiscall, mut_008B1270(this: u32) -> u32 {
    unsafe { update_coefficients(this, true) }
});
