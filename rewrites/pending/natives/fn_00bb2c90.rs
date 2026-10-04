// original: 0x00bb2c90 SET_PLAYER_MP_MODIFIER
/// Script native `SET_PLAYER_MP_MODIFIER` (hash 0x2B111E69).
///
/// Forwards a player index, a modifier index and one float bit-pattern
/// to the engine, which writes a multiplayer modifier. No return slot is
/// written.
export!(cdecl, rw_00bb2c90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
