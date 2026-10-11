// original: 0x00A74160 CTaskSimplePauseSystemTimer::vf17

/// Activates or refreshes the task's pause timer, then reports whether its
/// end time is at or before the current timer value.
///
/// The active byte is at `+0x1c`; a pending-refresh byte is at `+0x1d`.
/// When inactive, the start time at `+0x14` is set to the current timer and
/// the saved duration at `+0x20` is copied to `+0x18`. A pending refresh
/// replaces the start time and clears `+0x1d`. The end comparison adds the
/// saved duration and start time with 32-bit wrapping, then compares the
/// result and current timer as signed 32-bit integers. One unused stack word
/// is popped; the result is returned in AL.
lf_checker_rt::export!(thiscall, rw_00a74160(this: u32, _unused: u32) -> u8 {
    unsafe {
        const SYSTEM_TIMER_VA: u32 = 0x0117_3594;
        const START_OFFSET: u32 = 0x14;
        const END_OFFSET: u32 = 0x18;
        const ACTIVE_OFFSET: u32 = 0x1C;
        const REFRESH_OFFSET: u32 = 0x1D;
        const DURATION_OFFSET: u32 = 0x20;

        let active = ((this + ACTIVE_OFFSET) as *const u8).read();
        if active == 0 {
            let duration = ((this + DURATION_OFFSET) as *const u32).read_unaligned();
            let now = lf_checker_rt::global::<u32>(SYSTEM_TIMER_VA).read();
            ((this + START_OFFSET) as *mut u32).write_unaligned(now);
            ((this + END_OFFSET) as *mut u32).write_unaligned(duration);
            ((this + ACTIVE_OFFSET) as *mut u8).write(1);
        }

        if ((this + ACTIVE_OFFSET) as *const u8).read() == 0 {
            return 0;
        }
        if ((this + REFRESH_OFFSET) as *const u8).read() != 0 {
            let now = lf_checker_rt::global::<u32>(SYSTEM_TIMER_VA).read();
            ((this + START_OFFSET) as *mut u32).write_unaligned(now);
            ((this + REFRESH_OFFSET) as *mut u8).write(0);
        }

        let end = ((this + END_OFFSET) as *const u32).read_unaligned();
        let start = ((this + START_OFFSET) as *const u32).read_unaligned();
        let now = lf_checker_rt::global::<u32>(SYSTEM_TIMER_VA).read();
        u8::from(end.wrapping_add(start) as i32 <= now as i32)
    }
});
