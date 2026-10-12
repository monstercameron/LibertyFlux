// original: 0x00DFBD90 sse_float_unary_tables_call
/// Computes a scalar tangent from the low word of XMM0 and returns the
/// target's resulting four-word XMM0 value.
///
/// Small finite inputs are scaled in double precision. Large finite inputs
/// use the scripted double helper result. Ordinary finite inputs use Rust's
/// double-precision tangent and round back to single precision. NaNs follow
/// the invalid arithmetic path while preserving the three upper input words.
use core::arch::x86::{_mm_set_ps, __m128};

const EXPONENT_MASK: u32 = 0x7F80_0000;
const EXPONENT_BIAS_WORD: u16 = 0x0080;
const FAST_EXPONENT_LIMIT: u16 = 0x46FF;
const SMALL_INPUT_SCALE_BITS: u64 = 0x3FF0_0000_0040_0000;

#[inline(always)]
fn result_vector(low: f32, upper: [u32; 3]) -> __m128 {
    unsafe {
        _mm_set_ps(
            f32::from_bits(upper[2]),
            f32::from_bits(upper[1]),
            f32::from_bits(upper[0]),
            low,
        )
    }
}

lf_checker_rt::export!(cdecl, rw_00dfbd90() -> __m128 {
    let bits = lf_checker_rt::xmm_word(0, 0);
    let input = f32::from_bits(bits);
    let magnitude = bits & 0x7FFF_FFFF;
    let high_word = (magnitude >> 16) as u16;
    let adjusted_exponent = high_word.wrapping_sub(EXPONENT_BIAS_WORD);

    if adjusted_exponent > FAST_EXPONENT_LIMIT {
        if (adjusted_exponent as i16) > FAST_EXPONENT_LIMIT as i16 {
            if magnitude & EXPONENT_MASK == EXPONENT_MASK {
                let invalid = core::hint::black_box(input) - core::hint::black_box(input);
                return result_vector(
                    invalid,
                    [
                        lf_checker_rt::xmm_word(0, 1),
                        lf_checker_rt::xmm_word(0, 2),
                        lf_checker_rt::xmm_word(0, 3),
                    ],
                );
            }

            let helper_argument = f64::from(input).to_bits();
            let helper_result = unsafe {
                lf_checker_rt::callee_cdecl!(
                    1,
                    f64,
                    helper_argument as u32,
                    (helper_argument >> 32) as u32,
                )
            };
            return result_vector(helper_result as f32, [0; 3]);
        }

        let input64 = f64::from(input);
        let scale = f64::from_bits(SMALL_INPUT_SCALE_BITS);
        let scaled = core::hint::black_box(input64) * core::hint::black_box(scale);
        return result_vector(
            scaled as f32,
            [
                (scaled.to_bits() >> 32) as u32,
                lf_checker_rt::xmm_word(0, 2),
                lf_checker_rt::xmm_word(0, 3),
            ],
        );
    }

    let ordinary = f64::from(input).tan();
    result_vector(
        ordinary as f32,
        [
            (f64::from(input).to_bits() >> 32) as u32,
            lf_checker_rt::xmm_word(0, 2),
            lf_checker_rt::xmm_word(0, 3),
        ],
    )
});
