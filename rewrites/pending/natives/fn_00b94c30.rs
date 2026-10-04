// original: 0x00b94c30 PAUSE_GAME
/// PAUSE_GAME: tail-jump thunk to a shared engine implementation.
/// Forwards the script call context and returns the callee's result.
lf_rn101_rt::export!(cdecl, rw_fn_00b94c30(ctx: u32) -> u32 {
    lf_rn101_rt::callee_cdecl!(1, u32, ctx,)
});
