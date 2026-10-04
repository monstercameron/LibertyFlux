// original: 0x00b87a70 SET_GAME_CAM_HEADING
/// Script native `SET_GAME_CAM_HEADING` (hash 0x45FB5CE1).
///
/// Forwards one script argument (a float bit-pattern) to the engine
/// worker. No return slot is written.
export!(cdecl, rw_00b87a70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
