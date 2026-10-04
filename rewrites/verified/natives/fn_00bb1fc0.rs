// original: 0x00bb1fc0 GET_PLAYER_RGB_COLOUR
/// Script native `GET_PLAYER_RGB_COLOUR` (hash 0x73BD71A9).
///
/// Forwards four script arguments (a player index and three out-pointers for
/// the colour components) to the engine. No return slot is written by the
/// handler itself.
export!(cdecl, rw_00bb1fc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
