// original: 0x005e7260 DRAW_SPRITE_FRONT_BUFF
/// Script native `DRAW_SPRITE_FRONT_BUFF` (hash 0x22417905).
///
/// Forwards nine script arguments to the engine: five float bit-patterns (position and size) followed by four integers. Floats are copied as raw bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_005e7260(ctx: *const u8) -> u32 {
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
        )
    }
});
