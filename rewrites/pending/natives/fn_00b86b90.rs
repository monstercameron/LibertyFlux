// original: 0x00b86b90 GET_CAM_NEAR_DOF
/// `GET_CAM_NEAR_DOF` (native hash `0x50D15F0D`): forward 2 script arguments to the
/// `NativeImpl_GET_CAM_NEAR_DOF_2` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00b86b90(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        let args = (*ctx).args;
        callee_cdecl!(1, u32, *args, *args.add(1));
    }
});
