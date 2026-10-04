// original: 0x00bb65e0 REGISTER_MULTIPLAYER_GAME_WIN
/// Script native `REGISTER_MULTIPLAYER_GAME_WIN`.
///
/// Forwards two script arguments (winner and game-type identifiers) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bb65e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
