// original: 0x00ba1e70 SET_DECISION_MAKER_ATTRIBUTE_LOW_HEALTH
/// `SET_DECISION_MAKER_ATTRIBUTE_LOW_HEALTH` (native hash `0x2FFA6C89`): forward 2 script arguments to the
/// `NativeImpl_SET_DECISION_MAKER_ATTRIBUTE_LOW_HEALTH` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00ba1e70(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        let args = (*ctx).args;
        callee_cdecl!(1, u32, *args, *args.add(1));
    }
});
