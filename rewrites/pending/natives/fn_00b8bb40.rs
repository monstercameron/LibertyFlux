// original: 0x00b8bb40 ADD_NEXT_MESSAGE_TO_PREVIOUS_BRIEFS
/// Script native `ADD_NEXT_MESSAGE_TO_PREVIOUS_BRIEFS` (hash 0x1B086D33).
///
/// Forwards one script argument (a flag) to the engine, coerced to 0/1.
/// No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. Reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00b8bb40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
