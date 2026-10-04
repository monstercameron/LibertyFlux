// original: 0x00bc7210 REQUEST_CAR_RECORDING
/// REQUEST_CAR_RECORDING: forward 1 script argument to the engine implementation.
lf_rn101_rt::export!(cdecl, rw_fn_00bc7210(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    lf_rn101_rt::callee_cdecl!(1, u32, a0,)
});
