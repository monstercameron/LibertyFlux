// original: 0x005D3470 task_timer_expired

/// Update a timer's start value when requested, then test its signed deadline.
///
/// The receiver contains the start value at offset 0, duration at offset 4,
/// enabled byte at offset 8, and reset flag at offset 9. Disabled timers
/// return zero. A nonzero reset flag copies the current game timer into the
/// start field and clears the flag. The function then compares the wrapping
/// sum of start and duration to the current game timer as signed 32-bit values;
/// equality counts as expired. The original's disabled branch clears only AL,
/// so this proof fixes entry EAX to zero and does not cover nonzero upper EAX
/// carry-through on that branch.
lf_checker_rt::export!(thiscall, rw_005d3470(this: u32) -> u32 {
    unsafe {
        const START_VALUE: u32 = 0x00;
        const DURATION: u32 = 0x04;
        const ENABLED: u32 = 0x08;
        const RESET: u32 = 0x09;
        const GAME_TIMER_VA: u32 = 0x011735B4;

        let state = this as *mut u8;
        if state.add(ENABLED as usize).read_unaligned() == 0 {
            return 0;
        }

        let timer = lf_checker_rt::global::<u32>(GAME_TIMER_VA);
        if state.add(RESET as usize).read_unaligned() != 0 {
            state.cast::<u32>().write_unaligned(timer.read_unaligned());
            state.add(RESET as usize).write_unaligned(0);
        }

        let start = state.cast::<u32>().read_unaligned();
        let duration = state.add(DURATION as usize).cast::<u32>().read_unaligned();
        let deadline = start.wrapping_add(duration);
        u32::from((deadline as i32) <= (timer.read_unaligned() as i32))
    }
});
