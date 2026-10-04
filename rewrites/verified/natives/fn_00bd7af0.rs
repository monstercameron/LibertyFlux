// original: 0x00BD7AF0 GET_NETWORK_JOIN_FAIL
/// F06 GET_NETWORK_JOIN_FAIL: no-arg call, stores low byte of answer.
export!(cdecl, rn10_get_network_join_fail(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, _) = ctx_parts(ctx);
        let answer: u32 = callee_cdecl!(2, u32,);
        *ret = answer & 0xFF;
        ret as u32
    }
});
