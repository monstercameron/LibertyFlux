// original: 0x00ba2680 SET_PED_WINDY_CLOTHING_SCALE
/// Script native `SET_PED_WINDY_CLOTHING_SCALE` (hash 0x12865550).
///
/// Forwards a character handle and one float value to the engine. The float is copied as raw bits. No return slot is written.
export!(cdecl, rw_00ba2680(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
