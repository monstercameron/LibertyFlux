// original: 0x00bd11d0 SET_CHAR_DROPS_WEAPONS_WHEN_DEAD
/// Script native `SET_CHAR_DROPS_WEAPONS_WHEN_DEAD` (hash 0x2D43113A).
///
/// Forwards a character handle and a boolean flag to the engine.

/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// No return slot is written.
export!(cdecl, rw_00bd11d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
