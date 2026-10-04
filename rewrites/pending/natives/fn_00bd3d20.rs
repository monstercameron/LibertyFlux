// original: 0x00bd3d20 DRAW_SPRITE_WITH_UV
/// Script native `DRAW_SPRITE_WITH_UV` (hash 0x58C41E8F).
///
/// Forwards ten script arguments to the engine sprite routine: a texture
/// handle followed by nine coordinate/UV words, copied as raw bits. No
/// return slot is written.
export!(cdecl, rw_00bd3d20(ctx: *const u8) -> u32 {
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
            *args.add(9)
        )
    }
});
