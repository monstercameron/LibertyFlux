// original: 0x00b871d0 SET_CAMERA_AUTO_SCRIPT_ACTIVATION
/// Script native `SET_CAMERA_AUTO_SCRIPT_ACTIVATION` (hash 0x31D53B3D).
///
/// Coerces one script argument to a bool and forwards it to the engine.
/// Quirk (observed): the handler writes the bool into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer; reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00b871d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let coerced = (ctx as u32 & 0xFFFF_FF00) | u32::from(*args != 0);
        callee_cdecl!(1, u32, coerced)
    }
});
