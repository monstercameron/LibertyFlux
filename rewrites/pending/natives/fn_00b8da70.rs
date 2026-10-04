// original: 0x00b8da70 SET_TEXT_TO_USE_TEXT_FILE_COLOURS
/// Script native `SET_TEXT_TO_USE_TEXT_FILE_COLOURS` (hash 0x52CE650B).
///
/// Forwards one boolean script argument (`arg != 0`) to the engine.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer; reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
export!(cdecl, rw_00b8da70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
