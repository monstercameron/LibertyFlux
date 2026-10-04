// original: 0x00bd3d80 DRAW_SPRITE_WITH_UV_COORDS
/// Script native `DRAW_SPRITE_WITH_UV_COORDS` (hash 0x2D1D17C9).
///
/// Forwards fourteen script arguments to the engine: a texture id, nine float
/// bit-patterns (position, size and UV coordinates) and four trailing words.
/// Floats are copied as raw bits, so the forward is bit-exact. No return
/// slot is written.
export!(cdecl, rw_00bd3d80(ctx: *const u8) -> u32 {
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
            *args.add(10),
            *args.add(11),
            *args.add(12),
            *args.add(13),
        )
    }
});
