// original: 0x00b8d170 PRINT_WITH_2_NUMBERS_BIG
/// Script native `PRINT_WITH_2_NUMBERS_BIG` (hash 0x43197215).
///
/// Prints a large frontend message with two numbers. Forwards five script
/// arguments (the text label, two numbers, and two display parameters) to
/// the engine. No return slot is written.
export!(cdecl, rw_00b8d170(ctx: *const u8) -> u32 {
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
