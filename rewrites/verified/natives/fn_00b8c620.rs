// original: 0x00B8C620 FLASH_ROUTE
/// Native handler `FLASH_ROUTE`.
///
/// Forward one coerced flag to the engine.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_k2_rt::export!(cdecl, rw_00b8c620(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let flag0 = u32::from(*args.add(0) != 0);
        lf_k2_rt::callee_cdecl!(1, u32, flag0)
    }
});
