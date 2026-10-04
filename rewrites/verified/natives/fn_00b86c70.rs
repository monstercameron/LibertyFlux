// original: 0x00b86c70 GET_GAME_VIEWPORT_ID
/// Script native `GET_GAME_VIEWPORT_ID` (hash 0x57F7558B).
///
/// Forwards one script argument (a viewport index) to the engine. No return slot is written.
export!(cdecl, rw_00b86c70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
