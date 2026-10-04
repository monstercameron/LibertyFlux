// original: 0x00b86a80 ENABLE_DEBUG_CAM
/// Script native `ENABLE_DEBUG_CAM` (hash 0x296B09E8).
///
/// Forwards one boolean script argument to the engine, coerced with
/// `arg != 0`. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The full dword is reproduced here
/// for bit-exact outgoing-call matching.
export!(cdecl, rw_00b86a80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
