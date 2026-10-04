// original: 0x00B93E60 ASCII_INT_TO_STRING
/// F01 ASCII_INT_TO_STRING: forwards args[0] to the engine, stores full answer.
export!(cdecl, rn10_ascii_int_to_string(ctx: *const u32) -> u32 {
    unsafe {
        let (ret, args) = ctx_parts(ctx);
        let answer: u32 = callee_cdecl!(2, u32, *args);
        *ret = answer;
        answer
    }
});
