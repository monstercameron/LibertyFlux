// original: 0x00ba1990 SET_CHAR_READY_TO_BE_EXECUTED
/// Script native `SET_CHAR_READY_TO_BE_EXECUTED` (hash 0x5F58606A).
///
/// Forwards two script arguments (a character handle and a boolean flag) to the engine. No return slot is written. The boolean flag is coerced with `arg != 0`.

/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00ba1990(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag1 = u32::from(*args.add(1) != 0);
        let quirked1 = (ctx as u32 & 0xFFFF_FF00) | flag1;
        callee_cdecl!(1, u32, *args, quirked1)
    }
});
