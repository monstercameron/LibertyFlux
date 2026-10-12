// original: 0x00b871a0 RESET_CAM_SPLINE_CUSTOM_SPEED_GRAPH
/// RESET_CAM_SPLINE_CUSTOM_SPEED_GRAPH: Resets a camera spline's custom speed graph.
/// Tail-jump thunk to a shared engine implementation: forwards the
/// script call context and returns the callee's result.
lf_checker_rt::export!(cdecl, rw_fn_00b871a0(ctx: u32) -> u32 {
    lf_checker_rt::callee_cdecl!(1, u32, ctx,)
});
