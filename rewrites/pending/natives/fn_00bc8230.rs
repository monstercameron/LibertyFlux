// original: 0x00bc8230 SWITCH_MAD_DRIVERS
/// Script native `SWITCH_MAD_DRIVERS`.
///
/// Forwards one boolean flag to the engine. The original coerces the flag
/// with `setne` into the low byte of its own incoming stack slot, so the
/// pushed word is `(ctx & !0xFF) | (arg != 0)`; this rewrite reproduces
/// that dword exactly from its own `ctx`. No return slot is written.
export!(cdecl, rw_00bc8230(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = ((ctx as u32) & !0xFF) | ((*args != 0) as u32);
        callee_cdecl!(1, u32, flag)
    }
});

