// original: 0x00bb1e90 GET_NUM_OF_MODELS_KILLED_BY_PLAYER
/// `GET_NUM_OF_MODELS_KILLED_BY_PLAYER` (native hash `0x75B43A72`): forward 3 script arguments to the
/// `NativeImpl_GET_NUM_OF_MODELS_KILLED_BY_PLAYER` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00bb1e90(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        let args = (*ctx).args;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
    }
});
