// original: 0x00DFB980 sse_float_unary_tables_a
/// Return the inverse cosine of the scalar value supplied in XMM0.
///
/// Domain endpoints and invalid values preserve the target's scalar-result
/// behavior; a small-input interval also preserves the input's upper words.
/// The ordinary range uses Rust's f32 inverse-cosine operation.
use core::arch::x86::{_mm_set_ps, __m128};

const ABS_MASK: u32 = 0x7FFF_FFFF;
const EXPONENT_MASK: u32 = 0x7F80_0000;
const ONE_BITS: u32 = 0x3F80_0000;
const INFINITY_BITS: u32 = 0x7F80_0000;

#[inline(always)]
fn result_vector(low: f32, high: [u32; 3]) -> __m128 {
    unsafe {
        _mm_set_ps(
            f32::from_bits(high[2]),
            f32::from_bits(high[1]),
            f32::from_bits(high[0]),
            low,
        )
    }
}

lf_checker_rt::export!(cdecl, rw_00dfb980() -> __m128 {
    let bits = lf_checker_rt::xmm_word(0, 0);
    let input = f32::from_bits(bits);
    let magnitude = bits & ABS_MASK;
    if magnitude & EXPONENT_MASK == EXPONENT_MASK && magnitude != INFINITY_BITS {
        let quieted = core::hint::black_box(input) + core::hint::black_box(0.0_f32);
        return result_vector(quieted, [quieted.to_bits(); 3]);
    }
    if magnitude > ONE_BITS {
        let invalid = 0.0_f32 * f32::INFINITY;
        return result_vector(invalid, [0; 3]);
    }
    if magnitude == ONE_BITS {
        return result_vector(input.acos(), [0; 3]);
    }

    let upper = if magnitude >= 0x3780_0000 && magnitude < 0x3D80_0000 {
        [
            lf_checker_rt::xmm_word(0, 1),
            lf_checker_rt::xmm_word(0, 2),
            lf_checker_rt::xmm_word(0, 3),
        ]
    } else {
        [0; 3]
    };
    result_vector(input.acos(), upper)
});
