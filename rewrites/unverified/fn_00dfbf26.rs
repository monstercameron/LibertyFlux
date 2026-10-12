// original: 0x00DFBF26 expf_sse
/// Compute the single-precision exponential with the image's range-reduction tables.
///
/// The routine uses a table-driven double-precision reduction for finite inputs
/// in its main interval. Very small magnitudes return the stored unit value;
/// finite magnitudes at or above the reduction limit saturate to infinity for
/// positive inputs and zero for negative inputs. Infinities use their stored
/// results, while NaNs are quieted by the original's self-addition path.
use core::arch::x86::{_mm_cvtss_si32, _mm_set_ps, _mm_set_ss, __m128};

const SCALE_F32_BITS: u32 = 0x46B8_AA3B;
const SCALE_F64_VA: u32 = 0x00F0_85D8;
const CORRECTION_F64_VA: u32 = 0x00F0_85E0;
const REDUCTION_TABLE_VA: u32 = 0x00F0_8558;
const EXPONENT_TABLE_A_VA: u32 = 0x00F0_8458;
const EXPONENT_TABLE_B_VA: u32 = 0x00F0_8358;
const POSITIVE_INFINITY_VA: u32 = 0x00F0_85E8;
const NEGATIVE_INFINITY_RESULT_VA: u32 = 0x00F0_85F0;
const UNIT_RESULT_VA: u32 = 0x00F0_85EC;
const TINY_MAGNITUDE_BITS: u32 = 0x3180_0000;
const REDUCTION_LIMIT_BITS: u32 = 0x4380_0000;
const INFINITY_MAGNITUDE_BITS: u32 = 0x7F80_0000;

#[inline(always)]
unsafe fn image_f64(file_va: u32) -> f64 {
    core::ptr::read_unaligned(lf_checker_rt::global::<f64>(file_va))
}

#[inline(always)]
unsafe fn image_f32(file_va: u32) -> f32 {
    core::ptr::read_unaligned(lf_checker_rt::global::<f32>(file_va))
}

#[inline(always)]
fn vector_result(low: f32, high_words: [u32; 3]) -> __m128 {
    unsafe {
        _mm_set_ps(
            f32::from_bits(high_words[2]),
            f32::from_bits(high_words[1]),
            f32::from_bits(high_words[0]),
            low,
        )
    }
}

lf_checker_rt::export!(cdecl, rw_00dfbf26() -> __m128 {
    let input_bits = lf_checker_rt::xmm_word(0, 0);
    let input = f32::from_bits(input_bits);
    let magnitude_bits = input_bits & 0x7FFF_FFFF;

    if magnitude_bits <= TINY_MAGNITUDE_BITS {
        let value = unsafe { image_f32(UNIT_RESULT_VA) };
        return vector_result(value, [0; 3]);
    }

    if magnitude_bits >= REDUCTION_LIMIT_BITS {
        if magnitude_bits >= INFINITY_MAGNITUDE_BITS {
            if magnitude_bits == INFINITY_MAGNITUDE_BITS {
                let va = if input_bits == INFINITY_MAGNITUDE_BITS {
                    POSITIVE_INFINITY_VA
                } else {
                    NEGATIVE_INFINITY_RESULT_VA
                };
                let value = unsafe { image_f32(va) };
                return vector_result(value, [0; 3]);
            }

            let quieted = core::hint::black_box(input) + core::hint::black_box(input);
            return vector_result(
                quieted,
                [
                    lf_checker_rt::xmm_word(0, 1),
                    lf_checker_rt::xmm_word(0, 2),
                    lf_checker_rt::xmm_word(0, 3),
                ],
            );
        }

        if input_bits & 0x8000_0000 == 0 {
            // Finite magnitudes at least 256 overflow the positive result.
            let max_finite = f32::from_bits(0x7F7F_FFFF);
            return vector_result(max_finite * max_finite, [0; 3]);
        }

        let min_normal = f32::from_bits(0x0080_0000);
        return vector_result(min_normal * min_normal, [0; 3]);
    }

    let scale = f32::from_bits(SCALE_F32_BITS);
    let scaled = core::hint::black_box(input) * core::hint::black_box(scale);
    // This path is reached only for |input| < 256, so |scaled| < 11800.
    // The conversion therefore stays within i32, and round_ties_even matches
    // the source's current-rounding-mode single-precision conversion.
    let rounded = unsafe { _mm_cvtss_si32(_mm_set_ss(scaled)) };

    let table_a_offset = ((rounded as u32).wrapping_shl(3)) & 0x78;
    let table_b_offset = (((rounded >> 1) as u32) & 0x1F) << 3;
    let table_c_offset = (((rounded >> 6) as u32) & 0x1F) << 3;

    unsafe {
        let input_f64 = input as f64;
        let scaled_f64 = image_f64(SCALE_F64_VA) * input_f64;
        let residual = scaled_f64 - f64::from(rounded);

        let reduction = image_f64(REDUCTION_TABLE_VA.wrapping_add(table_a_offset));
        let correction = image_f64(CORRECTION_F64_VA) * reduction;
        let correction = correction * residual;
        let reduced = reduction + correction;

        let exponent_a = image_f64(EXPONENT_TABLE_A_VA.wrapping_add(table_b_offset));
        let exponent_b = image_f64(EXPONENT_TABLE_B_VA.wrapping_add(table_c_offset));
        let exponent_product = exponent_a * exponent_b;

        // The source performs packed 32-bit integer addition after shifting
        // the signed high exponent into the double's exponent field.
        let exponent_adjustment = ((rounded >> 14) as u64) << 52;
        let exponent_bits = exponent_product.to_bits();
        let adjusted_bits = (exponent_bits as u32)
            .wrapping_add(exponent_adjustment as u32) as u64
            | (((exponent_bits >> 32) as u32)
                .wrapping_add((exponent_adjustment >> 32) as u32) as u64)
                << 32;
        let exponent = f64::from_bits(adjusted_bits);
        let output = (reduced * exponent) as f32;
        vector_result(output, [0; 3])
    }
});
