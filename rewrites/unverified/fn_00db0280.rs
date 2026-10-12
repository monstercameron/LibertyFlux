// original: 0x00DB0280 dequantize_signed_float_range

/// Map an unsigned 64-bit quantized value into a single-precision range.
///
/// The first argument is read as low and high 32-bit words; the remaining
/// arguments are the minimum, maximum, and bit count. The original converts
/// the full unsigned integer to `f32`, divides by `(1 << bit_count) - 1` using
/// the processor's five-bit shift count, then evaluates
/// `minimum + ratio * (maximum - minimum)` in that order. It returns the result
/// in ST(0); its temporary writes into incoming stack slots are not outputs.
lf_checker_rt::export!(cdecl, rw_00db0280(packed_value: u64, minimum: f32, maximum: f32, bit_count: u32) -> f32 {
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

    let shift = bit_count & 31;
    let mask = 1u32.wrapping_shl(shift).wrapping_sub(1);
    let denominator = (mask as i32) as f32;
    let normalized = divide_f32(packed_value as f32, denominator);
    let range = subtract_f32(maximum, minimum);
    let scaled = multiply_f32(normalized, range);
    add_f32(scaled, minimum)
});
