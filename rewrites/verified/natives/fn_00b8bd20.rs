// original: 0x00b8bd20 CHANGE_BLIP_NAME_TO_PLAYER_NAME
/// Script native `CHANGE_BLIP_NAME_TO_PLAYER_NAME` (hash 0x731B11A7).
///
/// Forwards two script arguments (a blip handle and a player index) to the
/// engine. No return slot is written.
export!(cdecl, rw_00b8bd20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
