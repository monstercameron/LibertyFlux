// original: 0x00b87690 SET_CAM_SPLINE_CUSTOM_SPEED_GRAPH
/// `SET_CAM_SPLINE_CUSTOM_SPEED_GRAPH` (native hash `0x391B5A76`): forward 1 script argument to the
/// `NativeImpl_SET_CAM_SPLINE_CUSTOM_SPEED_GRAPH_2` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00b87690(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        let args = (*ctx).args;
        callee_cdecl!(1, u32, *args);
    }
});
