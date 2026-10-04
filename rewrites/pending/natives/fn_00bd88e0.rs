// original: 0x00bd88e0 NETWORK_IS_RENDEZVOUS
// Rewrite of the NETWORK_IS_RENDEZVOUS native handler.

/// Script native `NETWORK_IS_RENDEZVOUS()`.
///
/// Takes no script arguments: calls the engine rendezvous check and stores
/// the zero-extended low byte of its answer in the context's return slot.
export!(cdecl, rw_bd88e0(ctx: *const u8) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let ret_slot = *(ctx as *const *mut u32);
        *ret_slot = answer & 0xFF;
        0
    }
});
