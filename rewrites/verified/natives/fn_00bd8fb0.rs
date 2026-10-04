// original: 0x00bd8fb0 REGISTER_KILL_IN_MULTIPLAYER_GAME
/// Script native `REGISTER_KILL_IN_MULTIPLAYER_GAME` (hash 0x7D6D0A6C).
///
/// Forwards three script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bd8fb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
