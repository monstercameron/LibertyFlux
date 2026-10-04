// original: 0x00b86ce0 GET_SCRIPT_CAM
/// GET_SCRIPT_CAM: Returns the script camera; forward 1 script argument to the engine implementation.
lf_rn109_rt::export!(cdecl, rw_fn_00b86ce0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    lf_rn109_rt::callee_cdecl!(1, u32, a0,)
});
