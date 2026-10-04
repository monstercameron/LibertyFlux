// original: 0x00bb1c30 CLEAR_PLAYER_HAS_DAMAGED_AT_LEAST_ONE_PED
/// Script native `CLEAR_PLAYER_HAS_DAMAGED_AT_LEAST_ONE_PED` (hash 0x45AB718F).
///
/// Forwards one script argument (a player index) to the engine. No return
/// slot is written; the engine answer is the exit value.
export!(cdecl, rw_00bb1c30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
