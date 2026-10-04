// original: 0x00b87c50 SET_SCREEN_FADE
/// Script native `SET_SCREEN_FADE` (hash 0x188E0FAC).
///
/// Forwards eleven script arguments (a boolean flag as the fourth, two float bit-patterns last, integers otherwise) to the engine. No return slot is written. The boolean flag is coerced with `arg != 0`.

/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00b87c50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag3 = u32::from(*args.add(3) != 0);
        let quirked3 = (ctx as u32 & 0xFFFF_FF00) | flag3;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            quirked3,
            *args.add(4),
            *args.add(5),
            *args.add(6),
            *args.add(7),
            *args.add(8),
            *args.add(9),
            *args.add(10),
        )
    }
});
