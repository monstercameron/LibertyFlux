// original: 0x00bb1e90 GET_NUM_OF_MODELS_KILLED_BY_PLAYER
/// `GET_NUM_OF_MODELS_KILLED_BY_PLAYER` (native hash `0x75B43A72`): forward 3 script arguments to the
/// `NativeImpl_GET_NUM_OF_MODELS_KILLED_BY_PLAYER` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00bb1e90(ctx: *const NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        __lf_callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
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
