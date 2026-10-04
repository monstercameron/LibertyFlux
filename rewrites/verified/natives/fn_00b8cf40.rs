// original: 0x00b8cf40 PRINT_HELP_FOREVER_WITH_NUMBER
/// Script native `PRINT_HELP_FOREVER_WITH_NUMBER` (hash 0x19836A5B).
///
/// Forwards two script arguments (a text id and a number) to the engine. No
/// return slot is written; the engine answer is the exit value.
export!(cdecl, rw_00b8cf40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
