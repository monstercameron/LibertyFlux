// original: 0x00b9f820 IS_CHAR_INJURED
/// Script native `IS_CHAR_INJURED` (hash 0x4ECB2267).
///
/// Forwards one script argument (a character handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9f820(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
