// original: 0x00bd3f80 FORCE_NOISE_OFF
/// Script native `FORCE_NOISE_OFF` (hash 0x0CC0186A).
///
/// Forwards one boolean flag (coerced with `arg != 0`) to the engine. No
/// return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00bd3f80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag0 = u32::from(*args != 0);
        let q0 = (ctx as u32 & 0xFFFF_FF00) | flag0;
        callee_cdecl!(
            1,
            u32,
            q0,
        )
    }
});
