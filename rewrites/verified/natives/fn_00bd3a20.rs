// original: 0x00bd3a20 DRAW_COLOURED_CYLINDER
/// Script native `DRAW_COLOURED_CYLINDER` (hash 0x309860C4).
///
/// Forwards nine script arguments to the debug-cylinder renderer: five
/// float bit-patterns (position and radius values) followed by four
/// integers (colour and flags). Floats are copied as raw bits, so the
/// forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bd3a20(ctx: *const u8) -> u32 {
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
