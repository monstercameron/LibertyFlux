// original: 0x00b94f40 SET_OVERRIDE_NO_SPRINTING_ON_PHONE_IN_MULTIPLAYER
/// Script native `SET_OVERRIDE_NO_SPRINTING_ON_PHONE_IN_MULTIPLAYER`
/// (hash 0x75F85826).
///
/// Forwards one script argument (a boolean flag) to the engine. The flag is
/// coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
export!(cdecl, rw_00b94f40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
