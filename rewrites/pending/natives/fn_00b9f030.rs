// original: 0x00b9f030 GET_CHAR_SPEED
/// GET_CHAR_SPEED: forward 2 script arguments to the engine implementation.
lf_rn101_rt::export!(cdecl, rw_fn_00b9f030(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let a1 = unsafe { *((args + 4) as *const u32) };
    lf_rn101_rt::callee_cdecl!(1, u32, a0, a1,)
});
