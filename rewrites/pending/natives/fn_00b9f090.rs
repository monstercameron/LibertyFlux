// original: 0x00b9f090 GET_CHAR_VELOCITY
/// `GET_CHAR_VELOCITY` (native hash `0x3B977FD4`): forward 4 script arguments to the
/// `NativeImpl_GET_CHAR_VELOCITY` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00b9f090(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        let args = (*ctx).args;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3));
    }
});
