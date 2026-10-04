// original: 0x00a96470 fade_clamp_and_advance
/// Clamps the channel cursor to the global limit, then runs one advance step.
///
/// A cursor of -1 means unstarted and is left alone; any other cursor above
/// the limit is pulled down to it. Returns the limit that was applied.
export!(thiscall, rw_00a96470(this: u32) -> u32 {
    unsafe {
        let limit = *(global::<u32>(0x11735D4) as *const u32);
        let cur = *((this + 4) as *const u32);
        if cur != 0xFFFF_FFFF && cur > limit {
            *((this + 4) as *mut u32) = limit;
        }
        callee_thiscall!(1, u32, this, limit);
        limit
    }
});
