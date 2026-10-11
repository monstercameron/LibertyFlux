// original: 0x009cb3a0 timer_value_in_half_open_range

/// Interpret the two arguments and the shared timing value as signed 32-bit integers. For a non-wrapping interval, return one when the shared value is at least the lower bound and below the upper bound. When the lower bound is greater, treat the interval as wrapping and accept values at or above the lower bound or below the upper bound.
lf_checker_rt::export!(cdecl, rw_009cb3a0(lower_inclusive: u32, upper_exclusive: u32) -> al {
    const CURRENT_VALUE_VA: u32 = 0x01295848;
    let current = unsafe { lf_checker_rt::global::<i32>(CURRENT_VALUE_VA).read_unaligned() };
    let lower = lower_inclusive as i32;
    let upper = upper_exclusive as i32;
    let matches = if lower <= upper {
        current >= lower && current < upper
    } else {
        current >= lower || current < upper
    };
    u32::from(matches)
});
