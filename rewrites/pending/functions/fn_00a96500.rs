// original: 0x00a96500 fade_advance_step
/// Advances the channel cursor toward the limit by the scaled rate.
///
/// Nothing happens when the channel's link is null. Otherwise the step is
/// the span (limit minus cursor) against the rate converted to an integer
/// with x86 truncation semantics (out of range or NaN becomes 0x80000000).
/// When the span overshoots, a live link target resets the channel while a
/// spent one retires the cursor to limit minus step.
export!(thiscall, rw_00a96500(this: u32, limit: u32) -> u32 {
    unsafe {
        let link = *((this + 0x1C) as *const u32);
        if link != 0 {
            let rate = (*((this + 8) as *const i32)) as f32 * (*((this + 0x0C) as *const f32));
            let span = limit.wrapping_sub(*((this + 4) as *const u32));
            // x86 cvttss2si: truncate toward zero; NaN, infinities and
            // out-of-range values yield 0x80000000 (Rust `as` saturates, so
            // the invalid cases are detected explicitly first).
            let need: i32 = if rate.is_nan() || rate >= 2147483648.0 || rate <= -2147483649.0 {
                0x8000_0000u32 as i32
            } else {
                rate as i32
            };
            if (span as i32) > need {
                if *(link as *const u8) != 0 {
                    *((this + 0x1C) as *mut u32) = 0;
                } else {
                    *((this + 4) as *mut u32) = limit.wrapping_sub(need as u32);
                }
            }
        }
        0
    }
});
