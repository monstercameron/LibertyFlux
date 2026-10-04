// original: 0x00BD8AD0 NETWORK_PLAYER_HAS_COMM_PRIVS
// Rewrite of the NETWORK_PLAYER_HAS_COMM_PRIVS native handler.

/// Script native `NETWORK_PLAYER_HAS_COMM_PRIVS()`.
///
/// Takes no script arguments: calls the engine comm-privileges check and
/// stores the zero-extended low byte of its answer in the context's return
/// slot.
export!(cdecl, rw_bd8ad0(ctx: *const u8) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let ret_slot = *(ctx as *const *mut u32);
        *ret_slot = answer & 0xFF;
        0
    }
});
