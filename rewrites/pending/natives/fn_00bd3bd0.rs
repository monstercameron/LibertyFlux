// original: 0x00bd3bd0 DRAW_RECT
/// Script native `DRAW_RECT` (hash 0x3B2526E3).
///
/// Forwards eight script arguments to the engine: four float bit-patterns
/// (rectangle geometry) followed by four integers (colour components).
/// Floats are copied as raw bits, so the forward is bit-exact. No return
/// slot is written.
export!(cdecl, rw_00bd3bd0(ctx: *const u8) -> u32 {
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
