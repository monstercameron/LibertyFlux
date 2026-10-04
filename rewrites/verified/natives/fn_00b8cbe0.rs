// original: 0x00B8CBE0 HAS_ADDITIONAL_TEXT_LOADED
/// F09 HAS_ADDITIONAL_TEXT_LOADED: 1 arg, low byte of answer.
export!(cdecl, rn10_has_additional_text_loaded(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, args) = ctx_parts(ctx);
        let answer: u32 = callee_cdecl!(2, u32, *args);
        *ret = answer & 0xFF;
        ret as u32
    }
});
