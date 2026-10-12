// original: 0x00932250 tree_imposter_fade_factor

/// Calculate the clamped fade factor for one cache record. `record_index` is
/// the zero-based index of a 64-byte record and `distance_scale` is the
/// caller-supplied floating-point divisor. The record's value at byte offset
/// `0x10` is normalized between two scaled global boundaries, clamped to the
/// global unit interval, then limited by the record's value at byte offset
/// `0x08`. Equal boundaries return exactly `1.0` without reading the record.
/// Floating-point comparisons are ordered: NaNs do not satisfy less-than or
/// greater-than, and equal signed zeros take the equal path. The function is
/// cdecl and returns its `f32` result through the x87 floating-point stack.
#[inline(always)]
fn mul_in_order(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
fn sub_in_order(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(always)]
fn div_in_order(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) / core::hint::black_box(right)
}

lf_checker_rt::export!(cdecl, rw_00932250(record_index: u32, distance_scale: f32) -> f32 {
    const RECORD_TABLE_VA: u32 = 0x011A_2A70;
    const RECORD_STRIDE: u32 = 64;
    const RECORD_LIMIT_OFFSET: u32 = 8;
    const RECORD_VALUE_OFFSET: u32 = 16;
    const BOUNDARY_START_VA: u32 = 0x0103_6BCC;
    const BOUNDARY_END_VA: u32 = 0x0103_6BD0;
    const BOUNDARY_SCALE_VA: u32 = 0x011A_2944;
    const SCALE_FACTOR_VA: u32 = 0x00FE_870C;
    const UNIT_LIMIT_VA: u32 = 0x00FE_88E8;

    let boundary_start = unsafe { lf_checker_rt::global::<f32>(BOUNDARY_START_VA).read_unaligned() };
    let boundary_end = unsafe { lf_checker_rt::global::<f32>(BOUNDARY_END_VA).read_unaligned() };
    if boundary_start == boundary_end {
        return 1.0;
    }

    let boundary_scale = unsafe { lf_checker_rt::global::<f32>(BOUNDARY_SCALE_VA).read_unaligned() };
    let scale_factor = unsafe { lf_checker_rt::global::<f32>(SCALE_FACTOR_VA).read_unaligned() };

    let scaled_start = mul_in_order(mul_in_order(boundary_start, boundary_scale), scale_factor);
    let scaled_end = mul_in_order(mul_in_order(boundary_end, boundary_scale), scale_factor);
    let record = lf_checker_rt::relocated(RECORD_TABLE_VA)
        .wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
    let record_value = unsafe {
        (record.wrapping_add(RECORD_VALUE_OFFSET) as *const f32).read_unaligned()
    };
    let normalized_value = div_in_order(record_value, distance_scale);
    let numerator = sub_in_order(normalized_value, scaled_start);
    let denominator = sub_in_order(scaled_end, scaled_start);
    let ratio = div_in_order(numerator, denominator);

    let unit_clamped = if ratio < 0.0 {
        0.0
    } else {
        let unit_limit = unsafe { lf_checker_rt::global::<f32>(UNIT_LIMIT_VA).read_unaligned() };
        if ratio > unit_limit { unit_limit } else { ratio }
    };
    let record_limit = unsafe {
        (record.wrapping_add(RECORD_LIMIT_OFFSET) as *const f32).read_unaligned()
    };
    if record_limit > unit_clamped { unit_clamped } else { record_limit }
});
