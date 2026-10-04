// original: 0x00b87a90 SET_GAME_CAM_PITCH
/// Script native `SET_GAME_CAM_PITCH` (hash 0x1BC772AC).
///
/// Forwards one float script argument (the pitch angle) as raw bits to the
/// engine. No return slot is written.
export!(cdecl, rw_00b87a90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
