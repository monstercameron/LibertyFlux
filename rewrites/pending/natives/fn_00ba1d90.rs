// original: 0x00ba1d90 SET_COMBAT_DECISION_MAKER
/// Script native handler `SET_COMBAT_DECISION_MAKER`.
///
/// Forwards the ped handle and decision-maker handle to the AI engine; no script return value.
lf_rn94_rt::export!(cdecl, rw_fn_00ba1d90(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let a0 = *((args + 0) as *const u32);
        let a1 = *((args + 4) as *const u32);
        let answer = lf_rn94_rt::callee_cdecl!(1, u32, a0, a1);
        answer
    }
});
