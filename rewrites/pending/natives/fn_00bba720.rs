// original: 0x00bba720 TASK_SHOOT_AT_CHAR
/// `TASK_SHOOT_AT_CHAR` (native hash `0x08022967`): forward 4 script arguments to the
/// `NativeImpl_TASK_SHOOT_AT_CHAR` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00bba720(ctx: *const crate::NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3));
    }
});
