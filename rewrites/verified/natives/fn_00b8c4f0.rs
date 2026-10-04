// original: 0x00b8c4f0 DRAW_CURVED_WINDOW_NOTEXT
/// Script native `DRAW_CURVED_WINDOW_NOTEXT` (hash 0x12B9197E).
///
/// Forwards four float bit-patterns (window geometry) and an integer to
/// the engine, which appends a textless curved-window entry. No return slot
/// is written.
export!(cdecl, rw_00b8c4f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4))
    }
});
