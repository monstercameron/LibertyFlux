// original: 0x00bba630 TASK_SET_CHAR_DECISION_MAKER
/// Script native handler `TASK_SET_CHAR_DECISION_MAKER`.
///
/// Forwards the character and decision-maker handles to the task engine; no script return value.
lf_rn94_rt::export!(cdecl, rw_fn_00bba630(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let a0 = *((args + 0) as *const u32);
        let a1 = *((args + 4) as *const u32);
        let answer = lf_rn94_rt::callee_cdecl!(1, u32, a0, a1);
        answer
    }
});
