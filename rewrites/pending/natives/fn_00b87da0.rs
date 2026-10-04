// original: 0x00b87da0 SET_VIEWPORT_DESTINATION
/// Script native `SET_VIEWPORT_DESTINATION` (hash 0x1C810358).
///
/// Forwards seven script arguments to the engine: an integer followed by
/// four float bit-patterns (moved through SSE registers) and two more
/// integers. Floats are copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00b87da0(ctx: *const u8) -> u32 {
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
        )
    }
});
