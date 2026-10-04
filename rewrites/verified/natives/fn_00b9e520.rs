// original: 0x00B9E520 CLEAR_CHAR_RELATIONSHIP
/// Clears a relationship between characters. Forwards three arguments to the engine helper.
///
/// Native handler: the script VM passes one pointer to a call context.
/// The context holds the return-slot pointer at offset 0 and the argument
/// array pointer at offset 8. Integer arguments pass through as raw words;
/// float arguments pass through bitwise.
lf_k2_rt::export!(cdecl, rw_00b9e520(ctx: *const u8) -> u32 {
    unsafe {
        let argv = *((ctx.add(8)) as *const *const u32);
        let a0 = *argv.add(0);
        let a1 = *argv.add(1);
        let a2 = *argv.add(2);
        lf_k2_rt::callee_cdecl!(1, u32, a0, a1, a2)
    }
});
