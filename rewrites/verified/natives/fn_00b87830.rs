// original: 0x00b87830 SET_CAR_MOTION_BLUR_EFFECT_BOAT
/// Script native `SET_CAR_MOTION_BLUR_EFFECT_BOAT` (hash 0x7D106167).
///
/// Forwards one script argument (a float bit-pattern) to the engine
/// worker. No return slot is written.
export!(cdecl, rw_00b87830(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
