// original: 0x00bd3cc0 DRAW_SPRITE_WITH_FIXED_ROTATION
/// Script native `DRAW_SPRITE_WITH_FIXED_ROTATION` (hash 0x7CB404D4).
///
/// Forwards ten script arguments to the engine: a texture handle, five float
/// bit-patterns (position, size and rotation values), and four integers.
/// Floats are copied as raw bits, so the forward is bit-exact. No return
/// slot is written.
export!(cdecl, rw_00bd3cc0(ctx: *const u8) -> u32 {
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
            *args.add(8),
            *args.add(9),
        )
    }
});
