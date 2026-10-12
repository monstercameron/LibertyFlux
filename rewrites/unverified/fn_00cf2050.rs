// original: 0x00CF2050 net_session_elapsed

/// Convert the elapsed high-resolution clock interval to truncated seconds.
///
/// The intercepted clock helper returns an unsigned 64-bit counter. The
/// starting counter is read from two image globals and subtracted with 64-bit
/// wraparound; that difference is interpreted as signed for the original
/// `fild` conversion. The result is rounded to `f32`, multiplied in source
/// order by the image-resident seconds-per-tick scale, then converted with
/// signed 64-bit truncation toward zero. Invalid or out-of-range conversions
/// return the x87 `fistp` indefinite value (`i64::MIN`); EAX receives its low
/// word. The function does not write memory and restores the x87 stack and
/// control word before returning.
lf_checker_rt::export!(cdecl, rw_fn_00cf2050() -> u32 {
    const CLOCK_CALLEE: u32 = 1;
    const START_LOW_VA: u32 = 0x0171_dad8;
    const START_HIGH_VA: u32 = 0x0171_dadc;
    const SCALE_VA: u32 = 0x017a_cd18;
    const I64_MIN_F32: f32 = -9_223_372_036_854_775_808.0_f32;
    const I64_MAX_EXCLUSIVE_F32: f32 = 9_223_372_036_854_775_808.0_f32;

    let current_counter = lf_checker_rt::callee_cdecl!(CLOCK_CALLEE, u64,);
    let start_low = unsafe { lf_checker_rt::global::<u32>(START_LOW_VA).read_unaligned() };
    let start_high = unsafe { lf_checker_rt::global::<u32>(START_HIGH_VA).read_unaligned() };
    let start_counter = (u64::from(start_high) << 32) | u64::from(start_low);
    let signed_ticks = current_counter.wrapping_sub(start_counter) as i64;
    let elapsed_seconds = signed_ticks as f32;
    let seconds_per_tick = unsafe { lf_checker_rt::global::<f32>(SCALE_VA).read_unaligned() };
    let scaled_seconds = core::hint::black_box(elapsed_seconds)
        * core::hint::black_box(seconds_per_tick);

    // The guards reproduce FISTP's signed-64 invalid result. The cast below
    // is reached only when its operand is finite and within the instruction's
    // signed destination range, so Rust's truncating cast is then equivalent.
    let elapsed_whole_seconds = if scaled_seconds.is_nan()
        || scaled_seconds >= I64_MAX_EXCLUSIVE_F32
        || scaled_seconds < I64_MIN_F32
    {
        i64::MIN
    } else {
        scaled_seconds as i64
    };
    elapsed_whole_seconds as u32
});
