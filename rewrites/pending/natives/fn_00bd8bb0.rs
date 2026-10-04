// original: 0x00bd8bb0 NETWORK_RETURN_TO_RENDEZVOUS_PENDING
/// Script native `NETWORK_RETURN_TO_RENDEZVOUS_PENDING` (hash 0x6A66149A).
///
/// Calls the engine worker with no arguments and stores the low byte of
/// its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd8bb0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
