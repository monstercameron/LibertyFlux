// original: 0x005e7300 ALLOW_EMERGENCY_SERVICES
/// Script native `ALLOW_EMERGENCY_SERVICES` (hash 0x69A72C50).
///
/// Writes `(arg0 != 0)` as a single byte to the engine's
/// emergency-services flag. No engine call is made and no return slot is
/// written. Returns the argument-array pointer, matching the original's
/// exit register on this path.
export!(cdecl, rw_005e7300(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        *global::<u8>(0x105c6ea) = u8::from(*args != 0);
        args as u32
    }
});
