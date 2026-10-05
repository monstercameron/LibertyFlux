// original: 0x00b93e20 ALLOW_THIS_SCRIPT_TO_BE_PAUSED
/// Script native `ALLOW_THIS_SCRIPT_TO_BE_PAUSED` (hash 0x3514533B).
///
/// Forwards one boolean script argument to the engine, coerced with
/// `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching. No return slot is written. (The original cleans
/// its one pushed argument with `(an instruction of the original)`; the effect on the stack pointer
/// is identical to the plain cdecl return here.)
export!(cdecl, rw_00b93e20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
