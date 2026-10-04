// original: 0x00b8d270 PRINT_WITH_5_NUMBERS
/// Script native `PRINT_WITH_5_NUMBERS`.
///
/// Forwards eight script arguments to the engine. No return slot is written.
export!(cdecl, rw_00b8d270(ctx: *const u8) -> u32 {
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
            *args.add(7),
        )
    }
});
