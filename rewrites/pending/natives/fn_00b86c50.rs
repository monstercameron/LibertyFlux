// original: 0x00b86c50 GET_GAME_CAM
/// Script native `GET_GAME_CAM` (hash 0x0B2A2801).
///
/// Forwards one script argument to the engine.
/// No return slot is written.
export!(cdecl, rw_00b86c50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args,)
    }
});
