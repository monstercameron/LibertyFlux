// original: 0x00b9df20 ALLOW_SCENARIO_PEDS_TO_BE_RETURNED_BY_NEXT_COMMAND
/// Script native `ALLOW_SCENARIO_PEDS_TO_BE_RETURNED_BY_NEXT_COMMAND` (hash 0x6EEE7E6C).
///
/// Forwards one script argument (an on/off flag) to the engine. The flag
/// is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
export!(cdecl, rw_00b9df20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
