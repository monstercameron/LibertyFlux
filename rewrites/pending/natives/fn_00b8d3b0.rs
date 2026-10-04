// original: 0x00b8d3b0 RENDER_LOADING_CLOCK
/// Script native `RENDER_LOADING_CLOCK` (hash 0x0CD70514).
///
/// Forwards 17 script argument(s) to the engine: 10 float bit-pattern(s), 7 integer(s).
/// Floats are copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00b8d3b0(ctx: *const u8) -> u32 {
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
            *args.add(14),
            *args.add(15),
            *args.add(16),
        )
    }
});
