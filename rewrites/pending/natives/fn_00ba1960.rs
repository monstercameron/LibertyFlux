// original: 0x00ba1960 SET_CHAR_PROVIDE_COVERING_FIRE
/// Script native `SET_CHAR_PROVIDE_COVERING_FIRE` (hash 0x1A827B2C).
///
/// Script arguments: 2 word(s).
///
/// Forwards arg0 (integer/handle), arg1 (boolean flag) to the engine routine.
/// Each flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces a flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// No return slot is written.
export!(cdecl, rw_00ba1960(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let arg0 = *args.add(0);
        let flag1 = u32::from(*args.add(1) != 0);
        let arg1 = (ctx as u32 & 0xFFFF_FF00) | flag1;
        callee_cdecl!(1, u32, arg0, arg1, )
    }
});
