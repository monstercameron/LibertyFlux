// original: 0x00B8BF00 CLEAR_THIS_PRINT
/// F02 CLEAR_THIS_PRINT: forwards args[0], no return slot use.
export!(cdecl, rn10_clear_this_print(ctx: *const u32) -> u32 {
    unsafe {
        let (_, args) = ctx_parts(ctx);
        callee_cdecl!(2, u32, *args);
        0
    }
});
