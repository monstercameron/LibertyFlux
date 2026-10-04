// original: 0x00a96430 fade_range_classify
/// Classifies the channel level against its high/low bounds (1, 2 or 3).
///
/// Returns 0 when the readiness probe fails. Otherwise a level above the
/// high bound is 1; a low bound above the high bound is 2, else 3.
/// Unordered (NaN) comparisons take the not-above branch, as with comiss.
export!(thiscall, rw_00a96430(this: u32) -> u32 {
    unsafe {
        if callee_thiscall!(1, u32, this) & 0xFF == 0 {
            return 0;
        }
        let hi = *((this + 0x14) as *const f32);
        let cur = *((this + 0x0C) as *const f32);
        if cur > hi {
            return 1;
        }
        let lo = *((this + 0x10) as *const f32);
        if lo > hi {
            2
        } else {
            3
        }
    }
});
