// original: 0x00BD81E0 NETWORK_CHECK_INVITE_ARRIVAL
/// Native handler `NETWORK_CHECK_INVITE_ARRIVAL`.
///
/// Call the engine with no arguments; store the full result.
///
/// The call is the first instruction; ctx is only read afterwards for the return slot.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_k2_rt::export!(cdecl, rw_00bd81e0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let ret_slot = *ctx_words as *mut u32;
        let answer: u32 = lf_k2_rt::callee_cdecl!(1, u32,);
        *ret_slot = answer;
        answer
    }
});
