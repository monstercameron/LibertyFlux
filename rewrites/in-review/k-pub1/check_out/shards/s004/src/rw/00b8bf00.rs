// original: 0x00B8BF00 CLEAR_THIS_PRINT
/// F02 CLEAR_THIS_PRINT: forwards args[0], no return slot use.
export!(cdecl, rn10_clear_this_print(ctx: *const u32) -> u32 {
    unsafe {
        let (_, args) = ctx_parts(ctx);
        __lf_callee_cdecl!(2, u32, *args);
        0
    }
});

/// Read the return slot and argument pointers from a call context. (r-n10.)
#[inline(always)]
pub unsafe fn ctx_parts(ctx: *const u32) -> (*mut u32, *const u32) {
    ((*ctx.add(RET_OFF_WORDS)) as *mut u32, (*ctx.add(ARGS_OFF_WORDS)) as *const u32)
}

/// Word offset of the argument-array pointer in the call context. (r-n10.)
pub const ARGS_OFF_WORDS: usize = 2;

/// Word offset of the return-slot pointer in the call context. (r-n10.)
pub const RET_OFF_WORDS: usize = 0;
