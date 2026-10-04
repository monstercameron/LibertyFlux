// original: 0x00bc7660 SET_CAR_LANE_SHIFT
/// SET_CAR_LANE_SHIFT: Sets a car's lane shift; forward 2 script arguments to the engine implementation.
/// Float arguments are forwarded as raw bits (bit-exact by construction).
lf_rn109_rt::export!(cdecl, rw_fn_00bc7660(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let a1 = unsafe { *((args + 4) as *const u32) }; // float bits
    lf_rn109_rt::callee_cdecl!(1, u32, a0, a1,)
});
