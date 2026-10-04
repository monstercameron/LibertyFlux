// original: 0x00b87710 SET_CAM_SPLINE_SPEED_GRAPH
/// Set a camera spline's speed graph.
///
/// Forwards the camera and graph ids (arguments 0-1) to the engine
/// implementation. Returns whatever the engine call returned.
export!(cdecl, rw_00b87710(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
