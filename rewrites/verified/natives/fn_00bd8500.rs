// original: 0x00bd8500 NETWORK_GET_PLAYER_ID_OF_NEXT_TEXT_CHAT
/// Script native `NETWORK_GET_PLAYER_ID_OF_NEXT_TEXT_CHAT` (hash 0x145B50AF).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer into the return slot. The engine call runs
/// before the context is read.
export!(cdecl, rw_00bd8500(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
