// original: 0x00ba1740 SET_CHAR_MELEE_MOVEMENT_CONSTAINT_BOX
/// Script native `SET_CHAR_MELEE_MOVEMENT_CONSTAINT_BOX` (hash 0x5A7D2C3C).
///
/// Forwards seven script arguments to the engine: a character handle
/// followed by six float bit-patterns (constraint-box bounds). The original
/// stages the floats through a stack temporary, but the callee receives
/// every word in script order, so the forward is a plain copy. Floats are
/// copied as raw bits, so the forward is bit-exact. No return slot is
/// written.
export!(cdecl, rw_00ba1740(ctx: *const u8) -> u32 {
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
