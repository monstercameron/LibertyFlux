// original: 0x00a1ea90 cam_flag_latch_step (proposed)

/// Steps a latched flag byte toward set or cleared.
///
/// `this` points to a record with a hold flag at `+HOLD_OFF` (bit 3) and
/// a latch byte at `+LATCH_OFF`. When the hold bit is set the latch is
/// cleared to 0 and 0 is returned. Otherwise a nonzero argument byte sets
/// the latch to 1 and returns 1; a zero argument leaves a non-positive
/// (signed) latch untouched returning 0, and decrements a positive latch
/// returning 1. Only `al` carries the result.
///
/// Original: 0x00a1ea90 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a1ea90(this: u32, arg: u32) -> u32 {
    unsafe {
        const HOLD_OFF: u32 = 0x38c;
        const LATCH_OFF: u32 = 0x392;
        const HOLD_BIT: u8 = 8;
        let latch = (this + LATCH_OFF) as *mut u8;
        if ((this + HOLD_OFF) as *const u8).read() & HOLD_BIT != 0 {
            latch.write(0);
            return 0;
        }
        if arg as u8 != 0 {
            latch.write(1);
            return 1;
        }
        let v = latch.read();
        if (v as i8) <= 0 {
            return 0;
        }
        latch.write(v - 1);
        1
    }
});
