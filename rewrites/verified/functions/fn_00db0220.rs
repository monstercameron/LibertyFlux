// original: 0x00DB0220 dequantize_unsigned_float_range

/// Convert an unsigned quantized integer into the supplied single-precision range.
///
/// `quantized_value` is interpreted as an unsigned 32-bit value, `minimum` and
/// `maximum` are the range endpoints, and `bit_count` selects the mask
/// `(1 << bit_count) - 1` with the processor's five-bit shift count. The
/// original adds a read-only table correction selected by the input sign bit,
/// divides by the mask, then evaluates `minimum + ratio * (maximum - minimum)`
/// in that order. Its result is returned in the x87 stack register.
lf_checker_rt::export!(cdecl, rw_00db0220(quantized_value: u32, minimum: f32, maximum: f32, bit_count: u32) -> f32 {
    const UNSIGNED_CORRECTION_TABLE_FILE_VA: u32 = 0x00FE8F50;

    #[inline(always)]
    fn add_f64(left: f64, right: f64) -> f64 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(always)]
    fn subtract_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(always)]
    fn divide_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) / core::hint::black_box(right)
    }
    #[inline(always)]
    fn multiply_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(always)]
    fn add_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    let correction_index = quantized_value >> 31;
    let correction_address = UNSIGNED_CORRECTION_TABLE_FILE_VA
        .wrapping_add(correction_index.wrapping_mul(8));
    let correction = unsafe {
        lf_checker_rt::global::<f64>(correction_address).read_unaligned()
    };
    let signed_value = (quantized_value as i32) as f64;
    let unsigned_value = add_f64(signed_value, correction) as f32;

    let shift = bit_count & 31;
    let mask = 1u32.wrapping_shl(shift).wrapping_sub(1);
    let denominator = (mask as i32) as f32;
    let ratio = divide_f32(unsigned_value, denominator);
    let range = subtract_f32(maximum, minimum);
    let scaled = multiply_f32(ratio, range);
    add_f32(scaled, minimum)
});
