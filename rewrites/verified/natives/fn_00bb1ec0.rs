// original: 0x00bb1ec0 GET_PLAYERS_LAST_CAR_NO_SAVE
/// Script native `GET_PLAYERS_LAST_CAR_NO_SAVE` (hash 0x12067E8D).
///
/// Forwards one script argument (a player index) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb1ec0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
