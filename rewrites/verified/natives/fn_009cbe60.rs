// original: 0x009cbe60 IS_PAIN_PLAYING
/// Script native `IS_PAIN_PLAYING` (hash 0x32422759).
///
/// Forwards one script argument (an audio handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_009cbe60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});

