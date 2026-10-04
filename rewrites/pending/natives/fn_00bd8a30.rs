// original: 0x00bd8a30 NETWORK_JOIN_SUMMONS
// Rewrite of the NETWORK_JOIN_SUMMONS native handler.

/// Script native `NETWORK_JOIN_SUMMONS()`.
///
/// Takes no script arguments: calls the engine join-summons routine and
/// stores the zero-extended low byte of its answer in the context's return
/// slot.
export!(cdecl, rw_bd8a30(ctx: *const u8) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let ret_slot = *(ctx as *const *mut u32);
        *ret_slot = answer & 0xFF;
        0
    }
});
