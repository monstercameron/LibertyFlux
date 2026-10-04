// original: 0x00b8c530 DRAW_CURVED_WINDOW_TEXT
/// Script native `DRAW_CURVED_WINDOW_TEXT` (hash 0x7DD67E15).
///
/// Forwards eight script arguments to the engine: three floats copied as
/// raw bits, then five integers. No return slot is written.
export!(cdecl, rw_00b8c530(ctx: *const u8) -> u32 {
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
