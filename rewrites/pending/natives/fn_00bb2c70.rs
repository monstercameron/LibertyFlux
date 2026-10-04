// original: 0x00bb2c70 SET_PLAYER_MOOD_PISSED_OFF
/// Script native `SET_PLAYER_MOOD_PISSED_OFF` (hash 0x5E061170).
///
/// Forwards two script arguments (a player index and a flag value) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bb2c70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
