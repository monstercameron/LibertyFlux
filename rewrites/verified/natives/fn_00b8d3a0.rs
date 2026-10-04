// original: 0x00b8d3a0 REMOVE_BLIP_AND_CLEAR_INDEX
/// REMOVE_BLIP_AND_CLEAR_INDEX: forward 1 script argument to the engine implementation.
lf_rn101_rt::export!(cdecl, rw_fn_00b8d3a0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    lf_rn101_rt::callee_cdecl!(1, u32, a0,)
});
