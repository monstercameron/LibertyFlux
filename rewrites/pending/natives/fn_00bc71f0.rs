// original: 0x00bc71f0 REMOVE_STUCK_CAR_CHECK
/// `REMOVE_STUCK_CAR_CHECK` (native hash `0x213308DB`): forward 1 script argument to the
/// `NativeImpl_REMOVE_STUCK_CAR_CHECK_2` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00bc71f0(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        let args = (*ctx).args;
        callee_cdecl!(1, u32, *args);
    }
});
