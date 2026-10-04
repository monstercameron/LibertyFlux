// original: 0x00b94ee0 SET_MESSAGES_WAITING
/// Script native `SET_MESSAGES_WAITING` (hash 0x7DC061F5).
///
/// Script arguments: 1 word(s).
///
/// Forwards arg0 (boolean flag) to the engine routine.
/// Each flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces a flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// No return slot is written.
export!(cdecl, rw_00b94ee0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag0 = u32::from(*args.add(0) != 0);
        let arg0 = (ctx as u32 & 0xFFFF_FF00) | flag0;
        callee_cdecl!(1, u32, arg0, )
    }
});
