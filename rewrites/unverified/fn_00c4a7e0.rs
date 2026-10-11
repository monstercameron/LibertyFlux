// original: 0x00C4A7E0 camera_spline_set_progress

/// Clamp a progress value, convert the scaled signed sample count using the
/// x86 truncating conversion rules, and store the remaining count.
///
/// `this` is in ECX and the one stack argument contains the progress as f32
/// bits. The signed count comes from offset 0x144. Progress below zero becomes
/// positive zero and values above one become one; unordered NaN comparisons
/// leave the input unchanged. The scaled value is converted with the x86
/// `cvttss2si` result, including `i32::MIN` for NaN or out-of-range input.
/// The global remaining count is read, reduced by the converted count, and
/// stored at receiver offset 0x14c. EAX retains the converted count on return.
lf_checker_rt::export!(thiscall, rw_c4a7e0(this: u32, progress_bits: u32) -> u32 {
    const COUNT_FIELD: u32 = 0x144;
    const REMAINING_FIELD: u32 = 0x14c;
    const GLOBAL_REMAINING_VA: u32 = 0x0117_35c4;
    const ZERO_BITS: u32 = 0;
    const ONE_BITS: u32 = 0x3f80_0000;
    const CVTT_UPPER_EXCLUSIVE: f32 = 2_147_483_648.0;
    const CVTT_LOWER_INCLUSIVE: f32 = -2_147_483_648.0;

    let input = f32::from_bits(progress_bits);
    let one = f32::from_bits(ONE_BITS);
    let progress = if input < 0.0 {
        f32::from_bits(ZERO_BITS)
    } else if input > one {
        one
    } else {
        input
    };
    let signed_count = unsafe { (this.wrapping_add(COUNT_FIELD) as *const i32).read_unaligned() };
    let count_as_float = signed_count as f32;
    let scaled_count = core::hint::black_box(count_as_float) * core::hint::black_box(progress);
    let converted_count = if !scaled_count.is_finite()
        || scaled_count >= CVTT_UPPER_EXCLUSIVE
        || scaled_count < CVTT_LOWER_INCLUSIVE
    {
        i32::MIN
    } else {
        // The checks above bound this value to the x86 truncation range.
        scaled_count as i32
    };
    let global_remaining = unsafe {
        lf_checker_rt::global::<u32>(GLOBAL_REMAINING_VA).read_unaligned()
    };
    let remaining = global_remaining.wrapping_sub(converted_count as u32);
    unsafe {
        (this.wrapping_add(REMAINING_FIELD) as *mut u32).write_unaligned(remaining);
    }
    converted_count as u32
});
