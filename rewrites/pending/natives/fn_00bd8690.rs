// original: 0x00BD8690 NETWORK_HOST_GAME_PENDING
// Rewrite of the NETWORK_HOST_GAME_PENDING native handler.

/// Script native `NETWORK_HOST_GAME_PENDING()`.
///
/// Takes no script arguments: calls the engine host-pending routine and
/// stores the zero-extended low byte of its answer in the context's return
/// slot.
export!(cdecl, rw_bd8690(ctx: *const u8) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let ret_slot = *(ctx as *const *mut u32);
        *ret_slot = answer & 0xFF;
        0
    }
});
