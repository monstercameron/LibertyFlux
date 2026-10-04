// original: 0x00b87a30 SET_FOV_CHANNEL_SCRIPT
/// SET_FOV_CHANNEL_SCRIPT: forward 1 script argument to the engine implementation.
/// Argument 0 is coerced to 0/1; the pushed word keeps the context
/// pointer's high bytes (the original reuses its incoming stack slot).
lf_rn101_rt::export!(cdecl, rw_fn_00b87a30(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let q0 = (ctx & 0xffff_ff00) | u32::from(a0 != 0);
    lf_rn101_rt::callee_cdecl!(1, u32, q0,)
});
