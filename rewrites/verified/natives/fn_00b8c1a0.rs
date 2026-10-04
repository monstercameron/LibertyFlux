// original: 0x00B8C1A0 DISPLAY_TEXT_SUBSTRING
/// Native handler `DISPLAY_TEXT_SUBSTRING`.
///
/// Forward two position floats, three ids and a coerced flag to the engine.
///
/// Arg order on the stack is (x, y, a2, flag, a4, a5, a6).
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_k2_rt::export!(cdecl, rw_00b8c1a0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let flag3 = u32::from(*args.add(3) != 0);
        lf_k2_rt::callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), flag3, *args.add(4), *args.add(5), *args.add(6))
    }
});
