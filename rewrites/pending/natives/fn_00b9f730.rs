// original: 0x00b9f730 IS_CHAR_DEAD
/// Script native `IS_CHAR_DEAD` (hash 0x6A6B4F18).
///
/// Forwards one script argument (a character handle) to the engine.
///
/// Stores the low byte of the engine answer (zero-extended)
/// into the return slot.
///
export!(cdecl, rw_00b9f730(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
