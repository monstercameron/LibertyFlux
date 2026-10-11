// original: 0x00ABDB50 input_ui_curve_weight

/// Compute the fourth power of the positive part of one minus a squared
/// ratio.
///
/// The two cdecl stack arguments are squared in order, the first square is
/// divided by the second, and the result is subtracted from the single-
/// precision constant 1.0. A non-positive or unordered remainder becomes
/// positive zero; otherwise it is multiplied by itself three more times.
/// Each operation is forced through scalar SSE so it rounds to f32 at the
/// same step as the original. The original temporarily overwrites its first
/// incoming argument slot with the return value before loading ST0; the
/// checker contract disables the stack-diff check and compares that value
/// through the exact ST0 return instead. The caller cleans both arguments.
lf_checker_rt::export!(cdecl, rw_00abdb50(numerator: f32, denominator: f32) -> f32 {
    #[inline(always)]
    fn mul_ss(left: f32, right: f32) -> f32 {
        use core::arch::x86::{_mm_cvtss_f32, _mm_mul_ss, _mm_set_ss};
        unsafe {
            _mm_cvtss_f32(_mm_mul_ss(
                _mm_set_ss(core::hint::black_box(left)),
                _mm_set_ss(core::hint::black_box(right)),
            ))
        }
    }

    #[inline(always)]
    fn div_ss(dividend: f32, divisor: f32) -> f32 {
        use core::arch::x86::{_mm_cvtss_f32, _mm_div_ss, _mm_set_ss};
        unsafe {
            _mm_cvtss_f32(_mm_div_ss(
                _mm_set_ss(core::hint::black_box(dividend)),
                _mm_set_ss(core::hint::black_box(divisor)),
            ))
        }
    }

    #[inline(always)]
    fn sub_ss(left: f32, right: f32) -> f32 {
        use core::arch::x86::{_mm_cvtss_f32, _mm_set_ss, _mm_sub_ss};
        unsafe {
            _mm_cvtss_f32(_mm_sub_ss(
                _mm_set_ss(core::hint::black_box(left)),
                _mm_set_ss(core::hint::black_box(right)),
            ))
        }
    }

    let numerator_squared = mul_ss(numerator, numerator);
    let denominator_squared = mul_ss(denominator, denominator);
    let ratio = div_ss(numerator_squared, denominator_squared);
    let remaining = sub_ss(1.0, ratio);
    let positive_part = if remaining > 0.0 { remaining } else { 0.0 };
    let square = mul_ss(positive_part, positive_part);
    let cube = mul_ss(square, positive_part);
    mul_ss(cube, positive_part)
});
