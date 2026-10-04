// original: 0x00a962c0 fade_progress_update
/// Refreshes the channel's progress ratio and clears the dirty flag.
///
/// The ratio is elapsed over span as a float when the span is positive and
/// the elapsed count is still inside it; otherwise the ratio clamps to 1.0
/// (or 0.0 when either input is negative) and, when the dirty flag is set
/// without the hold flag, the channel is reset before the dirty flag clears.
export!(thiscall, rw_00a962c0(this: u32) -> u32 {
    unsafe {
        let total = callee_thiscall!(1, u32, this);
        let done = *((this + 4) as *const u32);
        let span = *((this + 8) as *const u32);
        let elapsed = total.wrapping_sub(done);
        if (span as i32) > 0 && (elapsed as i32) < (span as i32) {
            *((this + 0x14) as *mut f32) = (elapsed as i32) as f32 / (span as i32) as f32;
        } else {
            if (span as i32) < 0 || (done as i32) < 0 {
                *((this + 0x14) as *mut u32) = 0;
            } else {
                *((this + 0x14) as *mut u32) = 0x3F80_0000;
            }
            let fl = *((this + 0x20) as *const u8);
            if fl & 2 != 0 && fl & 4 == 0 {
                callee_thiscall!(2, u32, this);
            }
            *((this + 0x20) as *mut u8) &= 0xFD;
        }
        0
    }
});
