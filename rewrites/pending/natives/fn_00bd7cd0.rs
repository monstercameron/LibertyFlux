// original: 0x00bd7cd0 GET_TEAM_RGB_COLOUR
/// Script native `GET_TEAM_RGB_COLOUR`.
///
/// Forwards four script arguments (a team index and three out-pointers for
/// the colour channels) to the engine. No return slot is written by the
/// handler itself.
export!(cdecl, rw_00bd7cd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
