// original: 0x00950A40 float_chain_converter
/// Convert the selected bucket to an unsigned-style float, scale and evaluate
/// it through an audio curve, truncate that result with x86 invalid-conversion
/// semantics, and use a double helper result to compute the returned integer.
///
/// A zero second argument selects the image's default float. Otherwise the
/// bucket helper's signed result is adjusted by one of two image table entries
/// before conversion to f32. The audio-curve call writes its scalar result over
/// the incoming second stack argument in the original; the rewrite carries the
/// value in a local. Both integer conversions return i32::MIN for NaN,
/// infinity, or a value outside the signed 32-bit range.
///
/// Original: `0x00950A40` (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00950a40(first_bits: u32, bucket_bits: u32) -> u32 {
    unsafe {
        const DEFAULT_FLOAT_VA: u32 = 0x00E7_58F8;
        const UNSIGNED_ADJUST_TABLE_VA: u32 = 0x00FE_8F50;
        const BUCKET_SCALE_VA: u32 = 0x00E8_AE9C;
        const AUDIO_CURVE_VA: u32 = 0x0116_8B3C;
        const DOUBLE_TWO_VA: u32 = 0x00FE_8A28;
        const RESULT_SCALE_VA: u32 = 0x00E7_8574;
        const ONE_VA: u32 = 0x00FE_88E8;

        let first_integer = first_bits as i32;
        let bucket_input = bucket_bits;
        let bucket_float = if bucket_input == 0 {
            lf_checker_rt::global::<f32>(DEFAULT_FLOAT_VA).read_unaligned()
        } else {
            let bucket: u32 = lf_checker_rt::callee_cdecl!(1, u32, bucket_input);
            let signed_bucket = bucket as i32 as f64;
            let sign_index = (bucket >> 31) as usize;
            let adjustment = (lf_checker_rt::global::<f64>(UNSIGNED_ADJUST_TABLE_VA)
                .add(sign_index))
                .read_unaligned();
            let adjusted = add64(signed_bucket, adjustment);
            core::hint::black_box(adjusted) as f32
        };

        let bucket_scale = lf_checker_rt::global::<f32>(BUCKET_SCALE_VA).read_unaligned();
        let curve_input = mul32(bucket_float, bucket_scale);
        let curve_this = lf_checker_rt::relocated(AUDIO_CURVE_VA);
        let curve_value: f32 = lf_checker_rt::callee_thiscall!(
            2,
            f32,
            curve_this,
            curve_input.to_bits(),
        );

        let first_truncated = cvttss2si(curve_value);
        let integer_scale_input = first_truncated as f32;
        let result_scale = lf_checker_rt::global::<f32>(RESULT_SCALE_VA).read_unaligned();
        let scaled_integer = mul32(integer_scale_input, result_scale);
        let scaled_integer_double = f64::from(scaled_integer);
        let double_two = lf_checker_rt::global::<f64>(DOUBLE_TWO_VA).read_unaligned();
        let answer_bits = call_double(double_two, scaled_integer_double);
        let answer = f64::from_bits(answer_bits);
        let answer_as_float = core::hint::black_box(answer) as f32;
        let one = lf_checker_rt::global::<f32>(ONE_VA).read_unaligned();
        let reciprocal = div32(one, answer_as_float);
        let first_as_float = first_integer as f32;
        let final_float = mul32(reciprocal, first_as_float);
        cvttss2si(final_float) as u32
    }
});

#[inline(never)]
fn call_double(two: f64, value: f64) -> u64 {
    let two_bits = two.to_bits();
    let value_bits = value.to_bits();
    lf_checker_rt::callee_cdecl!(
        3,
        u64,
        two_bits as u32,
        (two_bits >> 32) as u32,
        value_bits as u32,
        (value_bits >> 32) as u32,
    )
}

#[inline(never)]
fn add64(left: f64, right: f64) -> f64 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(never)]
fn mul32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(never)]
fn div32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) / core::hint::black_box(right)
}

#[inline(always)]
fn cvttss2si(value: f32) -> i32 {
    const I32_MIN: f32 = -2_147_483_648.0;
    const I32_MAX_EXCLUSIVE: f32 = 2_147_483_648.0;
    if !value.is_finite() || value < I32_MIN || value >= I32_MAX_EXCLUSIVE {
        i32::MIN
    } else {
        value.trunc() as i32
    }
}
