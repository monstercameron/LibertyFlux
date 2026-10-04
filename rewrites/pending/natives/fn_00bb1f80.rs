// original: 0x00bb1f80 GET_PLAYER_MAX_HEALTH
/// Script native `GET_PLAYER_MAX_HEALTH` (hash 0x52F27084).
///
/// Forwards two script arguments (player index, out-pointer) to the engine. Writes no return slot itself.
export!(cdecl, rw_00bb1f80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
