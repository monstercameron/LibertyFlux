// original: 0x00bb75e0 SET_DITCH_POLICE_MODELS
/// Script native `SET_DITCH_POLICE_MODELS` (hash 0x25AC586E).
///
/// Forwards one script argument (a boolean flag) to the engine.
///
/// No return slot is written.
///
/// Quirk (observed): the handler coerces the boolean argument into
/// the low byte of its own incoming stack slot and pushes the whole
/// dword, so the pushed word's high bytes repeat the context pointer.
/// The engine reads only the low byte (Inferred); the full dword is
/// reproduced here for bit-exact outgoing-call matching.
/// No return slot is written.
///
export!(cdecl, rw_00bb75e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
