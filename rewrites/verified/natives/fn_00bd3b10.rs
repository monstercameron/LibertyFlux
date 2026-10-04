// original: 0x00bd3b10 DRAW_LIGHT_WITH_RANGE
/// Script native `DRAW_LIGHT_WITH_RANGE` (hash 0x30D27EB1).
///
/// Forwards eight script arguments to the engine: three float bit-patterns
/// (position), three integers (colour), and two more float bit-patterns
/// (including the range). Floats are copied as raw bits, so the forward is
/// bit-exact. No return slot is written.
export!(cdecl, rw_00bd3b10(ctx: *const u8) -> u32 {
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
