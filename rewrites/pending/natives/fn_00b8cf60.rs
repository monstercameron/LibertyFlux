// original: 0x00b8cf60 PRINT_HELP_FOREVER_WITH_STRING
/// Script native `PRINT_HELP_FOREVER_WITH_STRING` (hash 0x36D60616).
///
/// Forwards two script arguments (a text label and a string) to the
/// engine. No return slot is written.
export!(cdecl, rw_00b8cf60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
