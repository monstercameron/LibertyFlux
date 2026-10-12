// original: 0x00DB0300 quantize_unsigned_float_range

/// Clamp a value to a range and encode it with an unsigned bit mask.
///
/// The arguments are the value, minimum, maximum, and bit count. Ordered
/// comparisons clamp below the minimum first and check the maximum only on the
/// other path; unordered comparisons leave the value unchanged, as in the
/// original's `jbe` branches. It computes `(value - minimum) / (maximum -
/// minimum)`, adds the loaded rounding bias divided by the mask, multiplies by
/// the mask, then truncates through a signed 64-bit x87 conversion and returns
/// its low word. The bias is read from the image's read-only data.
lf_checker_rt::export!(cdecl, rw_00db0300(value: f32, minimum: f32, maximum: f32, bit_count: u32) -> u32 {
    const ROUNDING_BIAS_FILE_VA: u32 = 0x00FE8830;
    const I64_MIN_F32: f32 = -9_223_372_036_854_775_808.0;
    const I64_LIMIT_F32: f32 = 9_223_372_036_854_775_808.0;

    #[inline(always)]
    fn subtract_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(always)]
    fn divide_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) / core::hint::black_box(right)
    }
    #[inline(always)]
    fn add_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(always)]
    fn multiply_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    let mut clamped = value;
    if minimum > clamped {
        clamped = minimum;
    } else if clamped > maximum {
        clamped = maximum;
    }

    let shift = bit_count & 31;
    let mask = 1u32.wrapping_shl(shift).wrapping_sub(1);
    let mask_as_f32 = (mask as i32) as f32;
    let value_offset = subtract_f32(clamped, minimum);
    let range = subtract_f32(maximum, minimum);
    let ratio = divide_f32(value_offset, range);
    let rounding_bias = unsafe {
        lf_checker_rt::global::<f32>(ROUNDING_BIAS_FILE_VA).read_unaligned()
    };
    let half_step = divide_f32(rounding_bias, mask_as_f32);
    let biased_ratio = add_f32(ratio, half_step);
    let quantized = multiply_f32(biased_ratio, mask_as_f32);

    // FISTP qword truncates and yields the indefinite value on NaN or values
    // outside signed 64-bit range; its low word is zero. The exact -2^63
    // boundary is valid but also has a zero low word. Rust's float cast is
    // used only after this bound check, matching the original conversion.
    if !quantized.is_finite() || quantized < I64_MIN_F32 || quantized >= I64_LIMIT_F32 {
        return 0;
    }
    (quantized as i64) as u32
});
