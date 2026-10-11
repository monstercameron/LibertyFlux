// original: 0x0097EB70 ped_task_has_unexpired_window

/// Returns whether either unsigned deadline is still within its one-second window.
///
/// The task stores two 32-bit timestamps at `+0x150` and `+0x14c`. The current
/// time is read from the task system's global clock. Each timestamp is advanced
/// by 1000 with 32-bit wrapping, then compared as an unsigned value. The result
/// is the low byte of the Boolean predicate.
lf_checker_rt::export!(thiscall, rw_0097EB70(this: u32) -> u8 {
    const FIRST_DEADLINE: u32 = 0x150;
    const SECOND_DEADLINE: u32 = 0x14C;
    const WINDOW_MILLISECONDS: u32 = 1000;
    const CURRENT_TIME_VA: u32 = 0x011735B4;
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    let current = unsafe { lf_checker_rt::global::<u32>(CURRENT_TIME_VA).read() };
    let first = unsafe { read_word(this.wrapping_add(FIRST_DEADLINE)) };
    let second = unsafe { read_word(this.wrapping_add(SECOND_DEADLINE)) };
    u8::from(
        current < first.wrapping_add(WINDOW_MILLISECONDS)
            || current < second.wrapping_add(WINDOW_MILLISECONDS),
    )
});