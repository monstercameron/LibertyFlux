// original: 0x00b87400 SET_CAM_INTERP_CUSTOM_SPEED_GRAPH
/// Script native `SET_CAM_INTERP_CUSTOM_SPEED_GRAPH` (hash 0x03102FEE).
///
/// Forwards one script argument (one float value) to the engine.
///
/// No return slot is written.
///
/// Float arguments are copied as raw bit patterns, so the forward
/// is bit-exact.
///
export!(cdecl, rw_00b87400(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
