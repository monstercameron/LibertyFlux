// original: 0x00b8bf10 CLEAR_THIS_PRINT_BIG_NOW
/// CLEAR_THIS_PRINT_BIG_NOW: clear a bigonscreen text slot.
///
/// Native handler. Forwards the slot handle to the text engine.
export!(cdecl, rw_00b8bf10(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        callee_cdecl!(1, u32, a0)
    }
});
