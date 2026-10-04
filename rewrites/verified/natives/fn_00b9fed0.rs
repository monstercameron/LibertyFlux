// original: 0x00B9FED0 IS_CHAR_TRYING_TO_ENTER_A_LOCKED_CAR
/// Native handler `IS_CHAR_TRYING_TO_ENTER_A_LOCKED_CAR`.
///
/// Forward a character handle; store the low byte of the result.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_k2_rt::export!(cdecl, rw_00b9fed0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let ret_slot = *ctx_words as *mut u32;
        let answer: u32 = lf_k2_rt::callee_cdecl!(1, u32, *args.add(0));
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
