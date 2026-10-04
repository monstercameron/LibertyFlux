// original: 0x00b8d1b0 PRINT_WITH_3_NUMBERS
/// `PRINT_WITH_3_NUMBERS` (native hash `0x5FE61572`): forward 6 script arguments to the
/// `NativeImpl_PRINT_WITH_3_NUMBERS` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00b8d1b0(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        let args = (*ctx).args;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5));
    }
});
