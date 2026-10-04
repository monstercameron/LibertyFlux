// original: 0x00b8d190 PRINT_WITH_2_NUMBERS_NOW
/// Script native `PRINT_WITH_2_NUMBERS_NOW` (hash 0x5D251D72).
///
/// Forwards five script arguments (a text label, display parameters and two
/// numbers) to the engine text queue. No return slot is written.
export!(cdecl, rw_00b8d190(ctx: *const u8) -> u32 {
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
