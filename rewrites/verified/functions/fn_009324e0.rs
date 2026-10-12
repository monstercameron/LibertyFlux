// original: 0x009324E0 scaled_square_buffer_size_float

/// Compute the byte size for a scaled square buffer. `count` is squared with
/// 32-bit wrapping, the low square is multiplied by 64, and its signed dword
/// interpretation is corrected with one of two image-resident `f64` values.
/// The corrected double is rounded to `f32`, multiplied by the image's scale
/// factor, and converted with truncation toward zero. The x87 conversion's
/// invalid result has a zero low dword, which is returned for NaN, infinity,
/// or values outside the signed 64-bit range. The low dword is then added to
/// eight times the wrapped square. The cdecl function takes one `u32` and
/// returns the wrapped result in EAX.
#[inline(always)]
fn add_f64_in_order(left: f64, right: f64) -> f64 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn mul_f32_in_order(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

/// Match a truncating x87 qword conversion: invalid input produces INT64_MIN,
/// whose low dword is zero. The bounds are checked before the only float-to-int
/// cast, so the cast receives a finite value in the signed 64-bit range.
#[inline(always)]
fn fistp_qword_low(value: f32) -> u32 {
    let wide = value as f64;
    const I64_MIN: f64 = -9_223_372_036_854_775_808.0;
    const I64_MAX_EXCLUSIVE: f64 = 9_223_372_036_854_775_808.0;
    if !value.is_finite() || wide < I64_MIN || wide >= I64_MAX_EXCLUSIVE {
        0
    } else {
        (value as i64) as u32
    }
}

lf_checker_rt::export!(cdecl, rw_009324E0(count: u32) -> u32 {
    const DOUBLE_CORRECTION_VA: u32 = 0x00FE_8F50;
    const SCALE_FACTOR_VA: u32 = 0x00FE_893C;

    let square = count.wrapping_mul(count);
    let scaled_square = square.wrapping_shl(6);
    let signed_scaled_square = (scaled_square as i32) as f64;
    let correction_index = scaled_square >> 31;
    let correction_address = DOUBLE_CORRECTION_VA.wrapping_add(correction_index * 8);
    let correction = unsafe {
        lf_checker_rt::global::<f64>(correction_address).read_unaligned()
    };
    let corrected_double = add_f64_in_order(signed_scaled_square, correction);
    let corrected_single = corrected_double as f32;
    let scale_factor = unsafe {
        lf_checker_rt::global::<f32>(SCALE_FACTOR_VA).read_unaligned()
    };
    let scaled_single = mul_f32_in_order(corrected_single, scale_factor);
    let truncated_low = fistp_qword_low(scaled_single);
    truncated_low.wrapping_add(square.wrapping_mul(8))
});
