// original: 0x00DB0390 quantize_signed_float_range

/// Quantize a clamped single-precision value through the 64-bit conversion helper.
///
/// The arguments are the value, minimum, maximum, and bit count. The function
/// uses the image's rounding bias and conversion threshold, performs the
/// single-precision normalization in source order, then calls the intercepted
/// x87 helper once. Values below the threshold pass through unchanged; values
/// at or above it are reduced by the threshold, and the middle band adds the
/// high bit to the helper's 64-bit result. The helper's own conversion behavior
/// is supplied by the checker contract.
lf_checker_rt::export!(cdecl, rw_00db0390(value: f32, minimum: f32, maximum: f32, bit_count: u32) -> u64 {
    const ROUNDING_BIAS_FILE_VA: u32 = 0x00FE8830;
    const CONVERSION_LIMIT_FILE_VA: u32 = 0x00FE8D10;
    const HIGH_HALF_BASE: u64 = 0x8000_0000_0000_0000;

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
    let bias_per_level = divide_f32(rounding_bias, mask_as_f32);
    let biased_ratio = add_f32(ratio, bias_per_level);
    let quantized = multiply_f32(biased_ratio, mask_as_f32);
    let conversion_limit = unsafe {
        lf_checker_rt::global::<f32>(CONVERSION_LIMIT_FILE_VA).read_unaligned()
    };

    if quantized < conversion_limit || quantized.is_nan() || conversion_limit.is_nan() {
        return lf_checker_rt::callee_cdecl!(1, u64, quantized.to_bits());
    }

    let remainder = subtract_f32(quantized, conversion_limit);
    if conversion_limit <= remainder || conversion_limit.is_nan() || remainder.is_nan() {
        return lf_checker_rt::callee_cdecl!(1, u64, remainder.to_bits());
    }

    let converted_remainder = lf_checker_rt::callee_cdecl!(1, u64, remainder.to_bits());
    converted_remainder.wrapping_add(HIGH_HALF_BASE)
});
