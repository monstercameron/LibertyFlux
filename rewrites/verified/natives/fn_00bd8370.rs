// original: 0x00BD8370 NETWORK_GET_HEALTH_RETICULE_OPTION
/// F15 NETWORK_GET_HEALTH_RETICULE_OPTION: no-arg call, low byte of answer.
export!(cdecl, rn10_network_get_health_reticule_option(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, _) = ctx_parts(ctx);
        let answer: u32 = callee_cdecl!(2, u32,);
        *ret = answer & 0xFF;
        ret as u32
    }
});
