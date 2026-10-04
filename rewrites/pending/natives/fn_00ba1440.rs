// original: 0x00ba1440 SET_CHAR_DIES_INSTANTLY_IN_WATER
/// Script native `SET_CHAR_DIES_INSTANTLY_IN_WATER` (hash 0x0CCA5CFC).
///
/// Forwards a character handle and a boolean flag to the engine. The flag
/// is coerced with `arg != 0` through the stack-slot quirk (see
/// `DISPLAY_FRONTEND_MAP_BLIPS`). No return slot is written.
export!(cdecl, rw_00ba1440(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
