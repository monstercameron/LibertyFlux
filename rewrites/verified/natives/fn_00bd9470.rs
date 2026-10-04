// original: 0x00bd9470 SET_ONLINE_SCORE
/// Script native `SET_ONLINE_SCORE` (hash 0x6B9C7392).
///
/// Forwards 2 script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bd9470(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
