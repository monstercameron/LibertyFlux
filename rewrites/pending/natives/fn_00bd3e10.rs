// original: 0x00bd3e10 DRAW_TOPLEVEL_SPRITE
// Rewrite of the DRAW_TOPLEVEL_SPRITE native handler.

/// Script native `DRAW_TOPLEVEL_SPRITE(...)` (ten arguments).
///
/// Forwards all ten script arguments, in order, to the engine sprite routine:
/// one leading integer, five floats, then four trailing integers. No return
/// slot is written; the engine's answer is left in the return register.
export!(cdecl, rw_bd3e10(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
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
