// original: 0x009cbbf0 GET_CUTSCENE_AUDIO_TIME_MS
/// Script native `GET_CUTSCENE_AUDIO_TIME_MS` (hash 0x2B8A0C6B).
///
/// Takes no script arguments: calls the engine with no arguments and stores
/// the full 32-bit answer (a millisecond timestamp) into the return slot.
/// The answer is the exit value.
export!(cdecl, rw_009cbbf0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
