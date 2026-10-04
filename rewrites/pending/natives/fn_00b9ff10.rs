// original: 0x00B9FF10 IS_CHAR_USING_SCENARIO
/// F12 IS_CHAR_USING_SCENARIO: 2 args, low byte of answer.
export!(cdecl, rn10_is_char_using_scenario(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, args) = ctx_parts(ctx);
        let answer: u32 = callee_cdecl!(2, u32, *args, *args.add(1));
        *ret = answer & 0xFF;
        ret as u32
    }
});
