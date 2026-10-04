// original: 0x00bb9cf0 TASK_GUARD_CURRENT_POSITION
/// TASK_GUARD_CURRENT_POSITION: forward 4 script arguments to the engine implementation.
/// Argument 3 is coerced to 0/1; the pushed word keeps the context
/// pointer's high bytes (the original reuses its incoming stack slot).
/// Float arguments are forwarded as raw bits (bit-exact by construction).
lf_rn101_rt::export!(cdecl, rw_fn_00bb9cf0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let a1 = unsafe { *((args + 4) as *const u32) }; // float bits
    let a2 = unsafe { *((args + 8) as *const u32) }; // float bits
    let a3 = unsafe { *((args + 12) as *const u32) };
    let q3 = (ctx & 0xffff_ff00) | u32::from(a3 != 0);
    lf_rn101_rt::callee_cdecl!(1, u32, a0, a1, a2, q3,)
});
