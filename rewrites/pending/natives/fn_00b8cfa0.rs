// original: 0x00b8cfa0 PRINT_HELP_FOREVER_WITH_TWO_NUMBERS
/// Script native `PRINT_HELP_FOREVER_WITH_TWO_NUMBERS` (hash 0x795227EE).
///
/// Forwards three script arguments (a text label and two numbers) to the
/// engine text queue. No return slot is written.
export!(cdecl, rw_00b8cfa0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
