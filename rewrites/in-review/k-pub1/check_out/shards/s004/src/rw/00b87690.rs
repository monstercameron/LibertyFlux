// original: 0x00b87690 SET_CAM_SPLINE_CUSTOM_SPEED_GRAPH
/// `SET_CAM_SPLINE_CUSTOM_SPEED_GRAPH` (native hash `0x391B5A76`): forward 1 script argument to the
/// `NativeImpl_SET_CAM_SPLINE_CUSTOM_SPEED_GRAPH_2` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00b87690(ctx: *const NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        __lf_callee_cdecl!(1, u32, *args);
    }
});

/// Native call context, r-n08 shape: return-slot pointer, reserved word,
/// argument-array pointer, argument count. (Renamed from r-n08's NativeCtx.)
#[repr(C)]
pub struct NativeCtx08 {
    pub ret_slot: *mut u32,
    pub reserved: u32,
    pub args: *const u32,
    pub argc: u32,
}
