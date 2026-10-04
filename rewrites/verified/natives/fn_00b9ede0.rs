// original: 0x00b9ede0 GET_CHAR_EXTRACTED_VELOCITY
/// Script native `GET_CHAR_EXTRACTED_VELOCITY` (hash 0x7B3F0058).
///
/// Forwards five script arguments to the engine: a character handle, a
/// boolean flag, and three trailing words.

/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// No return slot is written.
export!(cdecl, rw_00b9ede0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            quirked,
            *args.add(2),
            *args.add(3),
            *args.add(4),
        );
        answer
    }
});
