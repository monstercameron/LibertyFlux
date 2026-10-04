// original: 0x009cc750 SET_VOICE_ID_FROM_HEAD_COMPONENT
/// Script native `SET_VOICE_ID_FROM_HEAD_COMPONENT` (hash 0x02794E6B).
///
/// Forwards three script arguments to the engine: two integers and a
/// boolean flag coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
export!(cdecl, rw_009cc750(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(2) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, *args.add(1), quirked)
    }
});
