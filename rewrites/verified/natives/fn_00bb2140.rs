// original: 0x00bb2140 GET_TRAIN_PLAYER_WOULD_ENTER
/// Script native `GET_TRAIN_PLAYER_WOULD_ENTER` (hash 0x30481141).
///
/// Forwards two script arguments (a player index and an output slot) to
/// the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00bb2140(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
