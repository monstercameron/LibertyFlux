// original: 0x00bd3e70 DRAW_WINDOW
/// Script native `DRAW_WINDOW` (hash 0x232642DE).
///
/// Forwards six script arguments to the engine: five float bit-patterns
/// (window rectangle and related values) and one trailing integer.
/// No return slot is written.
export!(cdecl, rw_00bd3e70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
        );
        answer
    }
});
