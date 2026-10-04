// original: 0x00b8daf0 SET_TIMER_BEEP_COUNTDOWN_TIME
/// Script native `SET_TIMER_BEEP_COUNTDOWN_TIME` (hash 0x66B93E8C).
///
/// Forwards two integer script arguments to the engine. No return slot
/// is written.
export!(cdecl, rw_00b8daf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
