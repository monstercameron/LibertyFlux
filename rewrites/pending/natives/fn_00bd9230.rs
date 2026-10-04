// original: 0x00bd9230 SET_IGNORE_SERVER_UPDATE
/// Script native `SET_IGNORE_SERVER_UPDATE` (hash 0x6B2F6234).
///
/// Forwards a handle word and a boolean flag to the engine. The flag is
/// coerced with `arg != 0` through the stack-slot quirk (see
/// `DISPLAY_FRONTEND_MAP_BLIPS`). No return slot is written.
export!(cdecl, rw_00bd9230(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
