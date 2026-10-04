// original: 0x00bd96f0 START_KILL_TRACKING
/// Script native `START_KILL_TRACKING` (hash 0x00EF143D).
///
/// Forwards two script arguments (a target handle and flags) to the engine. No return slot is written.
export!(cdecl, rw_00bd96f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
