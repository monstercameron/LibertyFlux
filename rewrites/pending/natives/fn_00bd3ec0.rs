// original: 0x00bd3ec0 ENABLE_DEFERRED_LIGHTING
/// Toggle deferred lighting: coerce the script argument to 0/1 and pass
/// it to the engine. The original coerces through a stack temporary that
/// leaves the context pointer's high bytes in the pushed dword, so the
/// engine receives those exact bits; only the low byte is significant.
export!(cdecl, rw_00bd3ec0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let flag = u32::from(*args != 0);
        callee_cdecl!(1, u32, (ctx as u32 & 0xFFFFFF00) | flag);
        0
    }
});
