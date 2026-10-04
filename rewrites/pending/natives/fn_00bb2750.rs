// original: 0x00bb2750 MAKE_PLAYER_SAFE_FOR_CUTSCENE
/// Script native `MAKE_PLAYER_SAFE_FOR_CUTSCENE` (hash 0x45852A03).
///
/// Forwards one script argument (a player index) to the engine. No return
/// slot is written.
export!(cdecl, rw_00bb2750(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
