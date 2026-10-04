// original: 0x00a20d00 cam_countdown_probe_gate (proposed)

/// Counts a gated probe down while its dual probe stays silent.
///
/// `this` holds a flag byte at `+FLAG_OFF` (bit 2 arms the gate), a
/// second flag at `+HI_OFF` (bit 7 forces an early exit) and a countdown
/// at `+COUNT_OFF`. When disarmed the function returns at once; when the
/// force bit is set the countdown is zeroed, the arm bit cleared and the
/// function returns. Otherwise the countdown is decremented (a non-positive
/// old value clears the arm bit and returns), a null `a0` clears the arm
/// bit and returns, and then the dual probe of `rw_00a1eb00` runs on `a0`:
/// a nonzero first result clears the arm bit, a zero first result runs the
/// second pair and keeps the arm bit only when it is also zero. Returns
/// nothing; the integer results stand in for the original's
/// convert-to-float-and-compare-against-zero, which is exactly a
/// not-equal-zero test on integer sources.
///
/// Original: 0x00a20d00 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a20d00(this: u32, a0: u32) -> u32 {
    unsafe {
        const GET_A: u32 = 1;
        const CONVERT: u32 = 2;
        const GET_B: u32 = 3;
        const FLAG_OFF: u32 = 0x38d;
        const HI_OFF: u32 = 0x38e;
        const COUNT_OFF: u32 = 0x354;
        const ARM_BIT: u8 = 4;
        const FORCE_BIT: u8 = 0x80;
        let flag = (this + FLAG_OFF) as *mut u8;
        if flag.read() & ARM_BIT == 0 {
            return 0;
        }
        if ((this + HI_OFF) as *const u8).read() & FORCE_BIT != 0 {
            flag.write(flag.read() & !ARM_BIT);
            ((this + COUNT_OFF) as *mut u32).write_unaligned(0);
            return 0;
        }
        let count = ((this + COUNT_OFF) as *const u32).read_unaligned();
        ((this + COUNT_OFF) as *mut u32).write_unaligned(count.wrapping_sub(1));
        if (count as i32) <= 0 || a0 == 0 {
            flag.write(flag.read() & !ARM_BIT);
            return 0;
        }
        let t1 = lf_checker_rt::callee_thiscall!(GET_A, u32, a0);
        let v1 = lf_checker_rt::callee_cdecl!(CONVERT, u32, t1);
        if v1 != 0 {
            flag.write(flag.read() & !ARM_BIT);
            return 0;
        }
        let t2 = lf_checker_rt::callee_thiscall!(GET_B, u32, a0);
        let v2 = lf_checker_rt::callee_cdecl!(CONVERT, u32, t2);
        if v2 != 0 {
            flag.write(flag.read() & !ARM_BIT);
        }
        0
    }
});
