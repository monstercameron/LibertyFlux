// original: 0x009cbc20 GET_PLAYER_RADIO_MODE
/// Script native `GET_PLAYER_RADIO_MODE` (hash 0x32795678).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer into the return slot. The engine call runs
/// before the context is read.
export!(cdecl, rw_009cbc20(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
