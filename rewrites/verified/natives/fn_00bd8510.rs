// original: 0x00bd8510 NETWORK_GET_RENDEZVOUS_HOST_PLAYER_ID
/// Script native `NETWORK_GET_RENDEZVOUS_HOST_PLAYER_ID` (hash 0x282D29FE).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores its full 32-bit answer (the host player id) into the return
/// slot.
export!(cdecl, rw_00bd8510(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
