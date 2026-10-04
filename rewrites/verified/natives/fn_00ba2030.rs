// original: 0x00ba2030 SET_GROUP_CHAR_DECISION_MAKER
/// SET_GROUP_CHAR_DECISION_MAKER: Assigns a decision maker to a group; forward 2 script arguments to the engine implementation.
lf_rn109_rt::export!(cdecl, rw_fn_00ba2030(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let a1 = unsafe { *((args + 4) as *const u32) };
    lf_rn109_rt::callee_cdecl!(1, u32, a0, a1,)
});
