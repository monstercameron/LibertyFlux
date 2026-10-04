// original: 0x00b8d110 PRINT_STRING_WITH_TWO_LITERAL_STRINGS
/// Script native `PRINT_STRING_WITH_TWO_LITERAL_STRINGS` (hash 0x19486759).
///
/// Forwards five script arguments (a text label, display parameters and two
/// literal strings) to the engine text queue. No return slot is written.
export!(cdecl, rw_00b8d110(ctx: *const u8) -> u32 {
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
