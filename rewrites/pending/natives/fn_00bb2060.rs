// original: 0x00BB2060 GET_TIME_SINCE_PLAYER_DROVE_AGAINST_TRAFFIC
/// F08 GET_TIME_SINCE_PLAYER_DROVE_AGAINST_TRAFFIC: 1 arg, full answer.
export!(cdecl, rn10_get_time_since_drove_against_traffic(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, args) = ctx_parts(ctx);
        let answer: u32 = callee_cdecl!(2, u32, *args);
        *ret = answer;
        answer
    }
});
