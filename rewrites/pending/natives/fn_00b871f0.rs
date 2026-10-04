// original: 0x00b871f0 SET_CAMERA_BEGIN_CAM_COMMANDS_REQUIRED
/// Script native `SET_CAMERA_BEGIN_CAM_COMMANDS_REQUIRED` (hash 0x03B12ED0).
///
/// Forwards one script argument, a boolean flag coerced with `arg != 0`,
/// to the engine. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00b871f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
