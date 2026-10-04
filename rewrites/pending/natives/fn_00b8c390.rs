// original: 0x00b8c390 DISPLAY_TEXT_WITH_STRING_AND_INT
/// Script native `DISPLAY_TEXT_WITH_STRING_AND_INT` (hash 0x369A4540).
///
/// Displays a formatted text slot: forwards five words (two float
/// bit-patterns for position, then text id, string and number words).
/// No return slot is written.
export!(cdecl, rw_00b8c390(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        )
    }
});
