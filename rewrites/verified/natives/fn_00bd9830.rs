// original: 0x00bd9830 UPDATE_PLAYER_LCPD_SCORE
/// Script native `UPDATE_PLAYER_LCPD_SCORE` (hash 0x49EC44CA).
///
/// Forwards two script arguments (a player index and the score delta) to
/// the engine. No return slot is written.
export!(cdecl, rw_00bd9830(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
