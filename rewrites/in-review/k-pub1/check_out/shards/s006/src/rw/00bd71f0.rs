// original: 0x00BD71F0 GET_CURRENT_DAY_OF_WEEK
/// F04 GET_CURRENT_DAY_OF_WEEK: calls engine with no args, stores full answer.
export!(cdecl, rn10_get_current_day_of_week(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, _) = ctx_parts(ctx);
        let answer: u32 = __lf_callee_cdecl!(2, u32,);
        *ret = answer;
        answer
    }
});

/// Read the return slot and argument pointers from a call context (r-n10).
#[inline(always)]
unsafe fn ctx_parts(ctx: *const u32) -> (*mut u32, *const u32) {
    ((*ctx.add(RET_OFF_WORDS)) as *mut u32, (*ctx.add(ARGS_OFF_WORDS)) as *const u32)
}

/// Byte offset of the argument-array pointer in the call context (words).
const ARGS_OFF_WORDS: usize = 2;

/// Byte offset of the return-slot pointer in the call context (words).
const RET_OFF_WORDS: usize = 0;
