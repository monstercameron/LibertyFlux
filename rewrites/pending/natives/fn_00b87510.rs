// original: 0x00b87510 SET_CAM_POINT_DAMPING_PARAMS

/// Native handler `SET_CAM_POINT_DAMPING_PARAMS`.
///
/// Set camera point damping parameters (id plus three floats).
/// Forwards 4 argument(s) to the engine and returns nothing.
export!(cdecl, rw_00b87510(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let arg2 = *args.add(2);
        let arg3 = *args.add(3);
        let _ = arg0;
        callee_cdecl!(1, u32, arg0, arg1, arg2, arg3);
        0
    }
});
