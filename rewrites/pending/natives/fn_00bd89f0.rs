// original: 0x00bd89f0 NETWORK_JOIN_GAME_PENDING
/// Script native `NETWORK_JOIN_GAME_PENDING` (hash 0x76C53927).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// Stores the low byte of the engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd89f0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
