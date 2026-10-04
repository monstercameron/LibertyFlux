// original: 0x005e7200 DRAW_SPRITE_PHOTO
/// Script native `DRAW_SPRITE_PHOTO` (hash 0x4BD4248E).
///
/// Forwards nine script arguments to the engine: five float bit-patterns
/// (photo rectangle) followed by four integers. Floats are copied as raw
/// bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_005e7200(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6), *args.add(7), *args.add(8))
    }
});
