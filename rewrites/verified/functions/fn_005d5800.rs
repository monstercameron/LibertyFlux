// original: 0x005d5800 CMissionCleanupNY::vf1

/// Refresh the mission-cleanup flag byte, then tail-dispatch to the shared
/// cleanup routine.
///
/// Sets the shared flag byte to `1` when the record's status byte at `+0x94`
/// is non-zero (zero compared, either value kept otherwise), then forwards
/// the object pointer and the record to the shared tail callee and returns
/// its answer.
///
/// Original: 0x005d5800 (thiscall, one stack word: the record pointer).
lf_checker_rt::export!(thiscall, rw_005d5800(this: u32, record: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x0105C6EA;
        const STATUS_OFF: u32 = 0x94;
        const TAIL_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let mut flag = rd8(lf_checker_rt::relocated(FLAG));
        if rd8(record + STATUS_OFF) != 0 {
            flag = 1;
        }
        (lf_checker_rt::relocated(FLAG) as *mut u8).write(flag);
        lf_checker_rt::callee_thiscall!(TAIL_CALLEE, u32, this, record)
    }
});
