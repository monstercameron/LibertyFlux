// original: 0x00bb2c30 SET_PLAYER_KEEPS_WEAPONS_WHEN_RESPAWNED
/// Script native `SET_PLAYER_KEEPS_WEAPONS_WHEN_RESPAWNED` (hash 0x6C321179).
///
/// Forwards one script argument (a player index) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb2c30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});

