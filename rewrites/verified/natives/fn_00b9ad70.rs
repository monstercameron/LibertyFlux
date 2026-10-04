// original: 0x00b9ad70 RESET_NETWORK_RESTART_NODE_GROUP_MAPPING
/// RESET_NETWORK_RESTART_NODE_GROUP_MAPPING: tail-jump thunk to a shared engine implementation.
/// Forwards the script call context and returns the callee's result.
lf_rn101_rt::export!(cdecl, rw_fn_00b9ad70(ctx: u32) -> u32 {
    lf_rn101_rt::callee_cdecl!(1, u32, ctx,)
});
