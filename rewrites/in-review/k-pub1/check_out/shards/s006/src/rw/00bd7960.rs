// original: 0x00BD7960 GET_DESTROYER_OF_NETWORK_ID
/// F05 GET_DESTROYER_OF_NETWORK_ID: forwards 2 args, stores full answer.
export!(cdecl, rn10_get_destroyer_of_network_id(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, args) = ctx_parts(ctx);
        let answer: u32 = __lf_callee_cdecl!(2, u32, *args, *args.add(1));
        *ret = answer;
        answer
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
