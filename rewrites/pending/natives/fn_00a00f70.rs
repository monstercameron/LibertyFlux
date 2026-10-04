// original: 0x00a00f70 GET_ROPE_HEIGHT_FOR_OBJECT
/// Forward both script arguments to the engine rope-height query. The
/// handler itself stores no return value; whatever the engine reports
/// travels out of band.
export!(cdecl, rw_00a00f70(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1));
        0
    }
});
