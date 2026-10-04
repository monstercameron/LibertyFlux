// original: 0x00ba1320 SET_CHAR_DEFENSIVE_AREA_ATTACHED_TO_CAR
/// Script native `SET_CHAR_DEFENSIVE_AREA_ATTACHED_TO_CAR` (hash 0x7191562B).
///
/// Forwards ten script arguments (two integers, seven float bit-patterns and a boolean flag) to the engine. No return slot is written. The boolean flag is coerced with `arg != 0`.

/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00ba1320(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag9 = u32::from(*args.add(9) != 0);
        let quirked9 = (ctx as u32 & 0xFFFF_FF00) | flag9;
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
            quirked9,
        )
    }
});
