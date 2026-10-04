// original: 0x00bd8f30 PLAYER_WANTS_TO_JOIN_NETWORK_GAME
/// Script native handler `PLAYER_WANTS_TO_JOIN_NETWORK_GAME`.
///
/// Forwards the player handle to the network engine and stores the boolean answer (low byte) in the script return slot.
lf_rn94_rt::export!(cdecl, rw_fn_00bd8f30(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let a0 = *((args + 0) as *const u32);
        let answer = lf_rn94_rt::callee_cdecl!(1, u32, a0);
        let slot = *(ctx as *const u32);
        *((slot) as *mut u32) = answer & 0xFF;
        slot
    }
});
