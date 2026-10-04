// original: 0x00b8d150 PRINT_WITH_2_NUMBERS
/// Script native `PRINT_WITH_2_NUMBERS` (hash 0x230A740F).
///
/// Forwards five script arguments (a text id and four words) to the
/// engine. No return slot is written.
export!(cdecl, rw_00b8d150(ctx: *const u8) -> u32 {
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
