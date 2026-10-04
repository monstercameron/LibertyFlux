// original: 0x00bc78e0 SET_ENABLE_RC_DETONATE_ON_CONTACT
/// Script native `SET_ENABLE_RC_DETONATE_ON_CONTACT` (hash 0x7BD06E31).
///
/// Forwards a boolean flag (`arg != 0`) to the engine. No return slot is
/// written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The full dword is reproduced here
/// for bit-exact outgoing-call matching.
export!(cdecl, rw_00bc78e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
