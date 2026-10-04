// original: 0x00b86800 CAM_SEQUENCE_WAIT
/// Wait for a camera sequence: forward both script arguments to the
/// engine. No return value.
export!(cdecl, rw_00b86800(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1));
        0
    }
});
