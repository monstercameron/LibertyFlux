// original: 0x009cbee0 IS_SCRIPTED_SPEECH_PLAYING
/// Script native `IS_SCRIPTED_SPEECH_PLAYING` (hash 0x12D71B44).
///
/// Forwards one script argument (a ped handle) to the engine.
///
/// Stores the low byte of the engine answer (zero-extended)
/// into the return slot.
///
export!(cdecl, rw_009cbee0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
