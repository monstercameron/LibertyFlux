// original: 0x00bb22f0 INCREASE_PLAYER_MAX_HEALTH
/// Script native `INCREASE_PLAYER_MAX_HEALTH` (hash 0x40A703A6).
///
/// Forwards 2 script arguments (a player index and a health amount) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb22f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
