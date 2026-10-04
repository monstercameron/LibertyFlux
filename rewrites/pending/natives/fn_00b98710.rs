// original: 0x00b98710 SET_TEXT_INPUT_ACTIVE
/// Script native `SET_TEXT_INPUT_ACTIVE` (hash 0x2A28684C).
///
/// Bool flag.
///
/// Quirk (observed): the handler coerces the boolean argument
/// into the low byte of its own incoming stack slot and pushes
/// the whole dword, so the pushed word's high bytes repeat the
/// context pointer. The engine reads only the low byte (Inferred);
/// the full dword is reproduced here for bit-exact outgoing-call
/// matching.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00b98710(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
