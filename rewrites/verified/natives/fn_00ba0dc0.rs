// original: 0x00ba0dc0 SEARCH_CRITERIA_REJECT_PEDS_WITH_FLAG_FALSE
/// SEARCH_CRITERIA_REJECT_PEDS_WITH_FLAG_FALSE: Sets a search criterion rejecting peds whose flag is false; forward 1 script argument to the engine implementation.
lf_rn109_rt::export!(cdecl, rw_fn_00ba0dc0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    lf_rn109_rt::callee_cdecl!(1, u32, a0,)
});
