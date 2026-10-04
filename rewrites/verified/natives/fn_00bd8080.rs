// original: 0x00bd8080 LCPD_HAS_BEEN_CONFIGURED
/// Script native `LCPD_HAS_BEEN_CONFIGURED` (hash 0x23254427).
///
/// Forwards no arguments to the engine.
/// Writes the zero-extended low byte of the engine answer into the return slot.
export!(cdecl, rw_00bd8080(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32,);
        let slot = (*(ctx as *const u32)) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
