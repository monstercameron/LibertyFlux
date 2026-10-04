// original: 0x00b867f0 CAM_SEQUENCE_STOP
/// CAM_SEQUENCE_STOP: Stops a camera sequence; forward 1 script argument to the engine implementation.
lf_rn109_rt::export!(cdecl, rw_fn_00b867f0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    lf_rn109_rt::callee_cdecl!(1, u32, a0,)
});
