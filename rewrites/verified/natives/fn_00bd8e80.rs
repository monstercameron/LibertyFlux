// original: 0x00bd8e80 NETWORK_STORE_SINGLE_PLAYER_GAME
/// Script native `NETWORK_STORE_SINGLE_PLAYER_GAME` (hash 0x08181609).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd8e80(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
