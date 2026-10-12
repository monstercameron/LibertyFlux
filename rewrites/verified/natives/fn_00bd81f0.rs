// original: 0x00bd81f0 NETWORK_CLEAR_INVITE_ARRIVAL
/// NETWORK_CLEAR_INVITE_ARRIVAL: tail-jump thunk to a shared engine implementation.
/// Forwards the script call context and returns the callee's result.
lf_checker_rt::export!(cdecl, rw_fn_00bd81f0(ctx: u32) -> u32 {
    lf_checker_rt::callee_cdecl!(1, u32, ctx,)
});
