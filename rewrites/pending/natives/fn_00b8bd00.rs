// original: 0x00b8bd00 CHANGE_BLIP_NAME_FROM_TEXT_FILE
/// Renames a blip from a text label. Forwards the blip handle and label argument to the engine helper.
///
/// Native handler: the script VM passes one pointer to a call context.
/// The context holds the return-slot pointer at offset 0 and the argument
/// array pointer at offset 8. Integer arguments pass through as raw words;
/// float arguments pass through bitwise.
checker_rt::export!(cdecl, rw_00b8bd00(ctx: *const u8) -> u32 {
    unsafe {
        let argv = *((ctx.add(8)) as *const *const u32);
        let a0 = *argv.add(0);
        let a1 = *argv.add(1);
        checker_rt::callee_cdecl!(1, u32, a0, a1)
    }
});
