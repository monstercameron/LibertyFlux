// original: 0x00bb2860 RESET_LOCAL_PLAYER_WEAPON_STAT
/// Script native `RESET_LOCAL_PLAYER_WEAPON_STAT` (hash 0x6C1344C6).
///
/// Forwards two script arguments (a player index and a weapon id) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bb2860(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
