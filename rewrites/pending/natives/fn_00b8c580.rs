// original: 0x00b8c580 DRAW_WINDOW_TEXT
/// Script native `DRAW_WINDOW_TEXT` (hash 0x3D0F5735).
///
/// Forwards six script arguments to the engine: three float bit-patterns (a screen position) followed by three integers (text and style). Floats are copied as raw bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00b8c580(ctx: *const u8) -> u32 {
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
        )
    }
});
