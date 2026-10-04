// original: 0x00b8d210 PRINT_WITH_4_NUMBERS
/// Script native `PRINT_WITH_4_NUMBERS` (hash 0x4D4F65AE).
///
/// Forwards seven script arguments (a text label, display parameters and four numbers) to the engine. No return slot is written.
export!(cdecl, rw_00b8d210(ctx: *const u8) -> u32 {
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
            *args.add(5),
            *args.add(6),
        )
    }
});
