// original: 0x00bc7500 SET_CAR_DISTANCE_AHEAD_MULTIPLIER
/// Script native handler `SET_CAR_DISTANCE_AHEAD_MULTIPLIER`.
///
/// Forwards the vehicle handle and distance multiplier to the traffic engine; no script return value.
lf_rn94_rt::export!(cdecl, rw_fn_00bc7500(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let a0 = *((args + 0) as *const u32);
        let a1 = *((args + 4) as *const u32);
        let answer = lf_rn94_rt::callee_cdecl!(1, u32, a0, a1);
        answer
    }
});
