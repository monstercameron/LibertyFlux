// original: 0x009cc380 REQUEST_AMBIENT_AUDIO_BANK
/// Script native `REQUEST_AMBIENT_AUDIO_BANK` (hash 0x754E1999).
///
/// Forwards one script argument (an audio-bank name hash) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_009cc380(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
