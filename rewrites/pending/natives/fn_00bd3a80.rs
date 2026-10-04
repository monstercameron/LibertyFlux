// original: 0x00bd3a80 DRAW_CORONA
/// Script native `DRAW_CORONA` (hash 0x39ED0C43).
///
/// Forwards nine script arguments to the engine: four float bit-patterns
/// (a position plus a size) followed by colour, texture and flag words.
/// No return slot is written; the engine answer is the exit value.
export!(cdecl, rw_00bd3a80(ctx: *const u8) -> u32 {
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
