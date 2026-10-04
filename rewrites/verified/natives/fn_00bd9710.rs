// original: 0x00bd9710 STOP_KILL_TRACKING
/// Script native `STOP_KILL_TRACKING` (hash 0x28CA0AFE).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bd9710(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
