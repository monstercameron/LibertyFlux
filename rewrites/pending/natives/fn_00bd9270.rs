// original: 0x00bd9270 SET_INVINCIBILITY_TIMER_DURATION
/// Script native `SET_INVINCIBILITY_TIMER_DURATION` (hash 0x3E4233F7).
///
/// Forwards one script argument (a duration in ticks) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bd9270(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
