// original: 0x00A73250 CTaskSimplePauseSystemTimer::vf5

/// Starts the task's pause interval at the current timer value.
///
/// The timer word is read through the checker runtime's relocated global
/// reference. It is copied to `this + 0x14`; `this + 0x18` becomes
/// `u32::MAX`, and the byte at `+0x1c` becomes one. Three unused stack words
/// are popped by thiscall cleanup. The return value is true in AL.
lf_checker_rt::export!(thiscall, rw_00a73250(this: u32, _arg0: u32, _arg1: u32, _arg2: u32) -> u8 {
    unsafe {
        const SYSTEM_TIMER_VA: u32 = 0x0117_3594;
        const TIMER_START_OFFSET: u32 = 0x14;
        const TIMER_END_OFFSET: u32 = 0x18;
        const ACTIVE_OFFSET: u32 = 0x1C;

        let now = lf_checker_rt::global::<u32>(SYSTEM_TIMER_VA).read();
        ((this + TIMER_START_OFFSET) as *mut u32).write_unaligned(now);
        ((this + TIMER_END_OFFSET) as *mut u32).write_unaligned(u32::MAX);
        ((this + ACTIVE_OFFSET) as *mut u8).write(1);
        1
    }
});
