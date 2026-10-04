// original: 0x00bb2180 GIVE_PLAYER_HELMET
/// Script native `GIVE_PLAYER_HELMET` (hash 0x463F190F).
///
/// Forwards two script arguments (a player index and helmet flags) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bb2180(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
