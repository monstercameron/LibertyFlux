// original: 0x00bb6470 INCREMENT_INT_STAT_NO_MESSAGE
/// Script native `INCREMENT_INT_STAT_NO_MESSAGE` (hash 0x29827605).
///
/// Forwards two script arguments (a statistic id and an increment) to the engine. No return slot is written.
export!(cdecl, rw_00bb6470(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
