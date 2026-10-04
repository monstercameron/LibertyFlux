// original: 0x00bc55c0 DELETE_MISSION_TRAIN
/// DELETE_MISSION_TRAIN: forward 1 script argument to the engine implementation.
lf_rn101_rt::export!(cdecl, rw_fn_00bc55c0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    lf_rn101_rt::callee_cdecl!(1, u32, a0,)
});
