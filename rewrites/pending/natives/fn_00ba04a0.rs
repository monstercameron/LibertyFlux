// original: 0x00ba04a0 LOCATE_CHAR_ANY_MEANS_CHAR_2D
/// Script native `LOCATE_CHAR_ANY_MEANS_CHAR_2D` (hash 0x18EA4926).
///
/// Forwards five script arguments to the engine: two integers (character
/// handles), two float bit-patterns (a radius pair) and a boolean flag
/// coerced with `arg != 0`. Stores the low byte of the engine answer
/// (zero-extended) into the return slot.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00ba04a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(4) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            quirked,
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
