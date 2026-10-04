// original: 0x009cbd60 IS_AMBIENT_SPEECH_DISABLED
/// Script native `IS_AMBIENT_SPEECH_DISABLED` (hash 0x563F4CC2).
///
/// Forwards one script argument (a character handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_009cbd60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
