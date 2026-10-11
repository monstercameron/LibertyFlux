// original: 0x00CB26C0 CTaskComplexGoToPointAndStandStillTimed::vf19

/// If the signed state word at offset 0x40 is nonnegative, store the current
/// game tick at offset 0x44, copy the state word to offset 0x48, and set the
/// byte at offset 0x4c. Then tail-call the shared state handler with this
/// object and the one forwarded stack argument, returning its result. Negative
/// state values skip the three stores but still call the handler.
lf_checker_rt::export!(thiscall, rw_00cb26c0(this: u32, argument: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x40;
        const START_TICK: u32 = 0x44;
        const SAVED_STATE: u32 = 0x48;
        const ACTIVE_FLAG: u32 = 0x4c;
        const GAME_TICK: u32 = 0x0117_35B4;
        let state = ((this + STATE) as *const u32).read_unaligned();
        if (state as i32) >= 0 {
            let tick = (lf_checker_rt::relocated(GAME_TICK) as *const u32).read_unaligned();
            ((this + START_TICK) as *mut u32).write_unaligned(tick);
            ((this + SAVED_STATE) as *mut u32).write_unaligned(state);
            ((this + ACTIVE_FLAG) as *mut u8).write_unaligned(1);
        }
        lf_checker_rt::callee_thiscall!(1, u32, this, argument)
    }
});
