// original: 0x00bd86b0 NETWORK_HOST_GAME_SUCCEEDED
/// Script native `NETWORK_HOST_GAME_SUCCEEDED` (hash 0x1CA77E94).
///
/// Takes no script arguments: calls the engine worker with no
/// arguments.
/// Stores the low byte of the engine answer (zero-extended)
/// into the return slot.
export!(cdecl, rw_00bd86b0(ctx: *const u8) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
