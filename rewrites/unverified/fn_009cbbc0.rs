// original: 0x009cbbc0 GET_AUDIBLE_MUSIC_TRACK_TEXT_ID
/// Script native `GET_AUDIBLE_MUSIC_TRACK_TEXT_ID` (hash 0x18246AC8).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_009cbbc0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32, );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
