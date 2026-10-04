// original: 0x00bb2a30 SET_PLAYER_COLOUR
/// Script native `SET_PLAYER_COLOUR` (hash 0x6C8F2EEE).
///
/// Forwards a player index and a colour word to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb2a30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
