// original: 0x00bd8630 NETWORK_HOST_GAME_E1
/// Native handler `NETWORK_HOST_GAME_E1`.
///
/// Forward six words with one coerced flag; store the low byte of the result.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_rn14_rt::export!(cdecl, rw_00bd8630(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let ret_slot = *ctx_words as *mut u32;
        let flag1 = u32::from(*args.add(1) != 0);
        let answer: u32 = lf_rn14_rt::callee_cdecl!(1, u32, *args.add(0), (ctx & 0xFFFF_FF00) | flag1, *args.add(2), *args.add(3), *args.add(4), *args.add(5));
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
