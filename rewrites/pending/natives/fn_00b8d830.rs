// original: 0x00b8d830 SET_RADAR_SCALE
/// Script native `SET_RADAR_SCALE` (hash 0x75ED39CF).
///
/// Forwards one float script argument (the radar scale) to the engine as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00b8d830(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args,)
    }
});
