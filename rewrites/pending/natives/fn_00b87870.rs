// original: 0x00b87870 SET_COLLIDE_WITH_PEDS
/// Script native `SET_COLLIDE_WITH_PEDS` (hash 0x5FDF1493).
///
/// Forwards one boolean script argument to the engine through the
/// stack-slot quirk (see `DISPLAY_FRONTEND_MAP_BLIPS`). No return slot is
/// written.
export!(cdecl, rw_00b87870(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
