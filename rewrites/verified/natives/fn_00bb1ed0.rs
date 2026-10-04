// original: 0x00bb1ed0 GET_PLAYER_CHAR
/// Script native `GET_PLAYER_CHAR` (hash 0x511454A9).
///
/// Forwards 2 script arguments (a player index and an out-pointer) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb1ed0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});

