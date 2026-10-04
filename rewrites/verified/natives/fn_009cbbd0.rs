// original: 0x009cbbd0 GET_AUDIO_ROOM_ID
/// Script native `GET_AUDIO_ROOM_ID` (hash 0x03AC3097).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_009cbbd0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
