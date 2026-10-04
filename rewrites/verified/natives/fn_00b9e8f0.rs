// original: 0x00b9e8f0 DAMAGE_CHAR
/// Script native `DAMAGE_CHAR` (hash 0x6045426E).
///
/// Decreases the characters health
///
/// Script arguments: self: Char, hitPoints: int, _p3: bool.
///
/// Forwards arg0 (integer/handle), arg1 (integer/handle), arg2 (boolean flag) to the engine routine.
/// Each flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces a flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// No return slot is written.
export!(cdecl, rw_00b9e8f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let flag2 = u32::from(*args.add(2) != 0);
        let arg2 = (ctx as u32 & 0xFFFF_FF00) | flag2;
        callee_cdecl!(1, u32, arg0, arg1, arg2, )
    }
});
