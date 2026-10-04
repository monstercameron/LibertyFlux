// original: 0x00bc7240 RESET_STUCK_TIMER
/// Script native `RESET_STUCK_TIMER` (hash 0x73260714).
///
/// Forwards two script arguments (a vehicle handle and a timer index) to
/// the engine. No return slot is written; the engine answer is the exit value.
export!(cdecl, rw_00bc7240(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
