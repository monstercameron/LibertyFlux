// original: 0x00b8d010 PRINT_HELP_WITH_STRING_NO_SOUND
/// Script native `PRINT_HELP_WITH_STRING_NO_SOUND` (hash 0x15734852).
///
/// Forwards a text label and a string reference to the engine. No return
/// slot is written.
export!(cdecl, rw_00b8d010(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
