// original: 0x00b9eae0 FLUSH_SCENARIO_BLOCKING_AREAS
/// FLUSH_SCENARIO_BLOCKING_AREAS: Flushes the scenario blocking areas.
/// Tail-jump thunk to a shared engine implementation: forwards the
/// script call context and returns the callee's result.
lf_rn109_rt::export!(cdecl, rw_fn_00b9eae0(ctx: u32) -> u32 {
    lf_rn109_rt::callee_cdecl!(1, u32, ctx,)
});
