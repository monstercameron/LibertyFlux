// original: 0x00b876d0 SET_CAM_SPLINE_PROGRESS
/// Script native `SET_CAM_SPLINE_PROGRESS` (hash 0x5A712F63).
///
/// Forwards two script arguments to the engine: an integer and a float
/// bit-pattern. No return slot is written.
export!(cdecl, rw_00b876d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
