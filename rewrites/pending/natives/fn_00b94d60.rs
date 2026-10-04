// original: 0x00b94d60 SET_CREDITS_TO_RENDER_BEFORE_FADE
/// Script native `SET_CREDITS_TO_RENDER_BEFORE_FADE` (hash 0x35FA026D).
///
/// Forwards one boolean script argument to the engine, coerced with `arg !=
/// 0`. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00b94d60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
