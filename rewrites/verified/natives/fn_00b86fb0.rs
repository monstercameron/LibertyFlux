// original: 0x00b86fb0 IS_VIEWPORT_ACTIVE
/// Script native `IS_VIEWPORT_ACTIVE` (hash 0x5D2B2A9A).
///
/// Forwards one script argument (a viewport handle) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b86fb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
