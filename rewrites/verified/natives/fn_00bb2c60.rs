// original: 0x00bb2c60 SET_PLAYER_MOOD_NORMAL
/// Script native `SET_PLAYER_MOOD_NORMAL` (hash 0x546F5326).
///
/// Forwards one script argument (a player index) to the engine. No return
/// slot is written.
export!(cdecl, rw_00bb2c60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
