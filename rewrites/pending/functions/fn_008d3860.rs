// original: 0x008D3860 quad_stage_submit
/// Runs four staged submit calls, then tail-forwards to the dispatcher.
///
/// Settles two preamble calls, drives three method calls on the shared
/// submit object (carrying 2, 0 and a global config word, then argument
/// 10, then a pair of 4s), then issues four identical-shape staged calls
/// that cross the pairs (0, 1), (0, 3), (2, 1) and (2, 3) with argument 4,
/// a -1.0 tag and the first word of the argument-9 object, and finally
/// tail-forwards all eleven arguments to the dispatcher. The remaining
/// frame shuffling is dead stores. Returns the dispatcher's answer.
lf_checker_rt::export!(cdecl, rb109_fn5(
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32,
    a6: u32, a7: u32, a8: u32, a9: u32, a10: u32,
) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_PRE_A: u32 = 1; // preamble (cdecl/2: 0, 0)
    const CAL_PRE_B: u32 = 2; // preamble (cdecl/1: 0)
    const CAL_CFG: u32 = 3; // configure (thiscall/3: 2, 0, config)
    const CAL_SEL: u32 = 4; // select (thiscall/1: argument 10)
    const CAL_MODE: u32 = 5; // mode set (cdecl/2: 4, 4)
    const CAL_STAGE: u32 = 6; // staged submit (cdecl/7)
    const CAL_FLUSH: u32 = 7; // flush (cdecl/0)
    const CAL_DONE: u32 = 8; // finish (thiscall/0)
    const CAL_DISPATCH: u32 = 9; // dispatcher, tail call (thiscall/11)

    // Globals (file VAs; resolved through the worker's image base).
    const G_OBJ: u32 = 0x011736C8; // shared submit object
    const G_CFG: u32 = 0x011736CC; // config word forwarded to CAL_CFG

    // Stage marker pushed as the second word of every staged call (-1.0f).
    const STAGE_TAG: u32 = 0xBF80_0000;

    unsafe {
        lf_checker_rt::callee_cdecl!(CAL_PRE_A, u32, 0, 0);
        lf_checker_rt::callee_cdecl!(CAL_PRE_B, u32, 0);
        let obj = *(lf_checker_rt::global::<u32>(G_OBJ));
        let cfg = *(lf_checker_rt::global::<u32>(G_CFG));
        lf_checker_rt::callee_thiscall!(CAL_CFG, u32, obj, 2, 0, cfg);
        lf_checker_rt::callee_thiscall!(CAL_SEL, u32, obj, a10);
        lf_checker_rt::callee_cdecl!(CAL_MODE, u32, 4, 4);
        let head = *(a9 as *const u32);
        lf_checker_rt::callee_cdecl!(CAL_STAGE, u32, a0, a1, a4, 0, 0, STAGE_TAG, head);
        lf_checker_rt::callee_cdecl!(CAL_STAGE, u32, a0, a3, a4, 0, 0, STAGE_TAG, head);
        lf_checker_rt::callee_cdecl!(CAL_STAGE, u32, a2, a1, a4, 0, 0, STAGE_TAG, head);
        lf_checker_rt::callee_cdecl!(CAL_STAGE, u32, a2, a3, a4, 0, 0, STAGE_TAG, head);
        lf_checker_rt::callee_cdecl!(CAL_FLUSH, u32,);
        lf_checker_rt::callee_thiscall!(CAL_DONE, u32, obj);
        lf_checker_rt::callee_thiscall!(
            CAL_DISPATCH, u32, obj, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10
        )
    }
});

