// original: 0x00b9e540 CLEAR_CHAR_DECISION_MAKER_EVENT_RESPONSE
/// CLEAR_CHAR_DECISION_MAKER_EVENT_RESPONSE: Clears a character decision-maker event response; forward 2 script arguments to the engine implementation.
lf_rn109_rt::export!(cdecl, rw_fn_00b9e540(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let a1 = unsafe { *((args + 4) as *const u32) };
    lf_rn109_rt::callee_cdecl!(1, u32, a0, a1,)
});
