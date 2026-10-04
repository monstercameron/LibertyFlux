// original: 0x00BD71F0 GET_CURRENT_DAY_OF_WEEK
/// F04 GET_CURRENT_DAY_OF_WEEK: calls engine with no args, stores full answer.
export!(cdecl, rn10_get_current_day_of_week(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, _) = ctx_parts(ctx);
        let answer: u32 = callee_cdecl!(2, u32,);
        *ret = answer;
        answer
    }
});
