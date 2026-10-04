// original: 0x00b87430 SET_CAM_INTERP_DETAIL_ROT_STYLE_QUATS
/// Script native `SET_CAM_INTERP_DETAIL_ROT_STYLE_QUATS` (hash 0x439C47D5).
///
/// Forwards one script argument (an interpolation style) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b87430(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
