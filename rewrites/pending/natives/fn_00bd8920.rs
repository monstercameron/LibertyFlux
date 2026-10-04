// original: 0x00bd8920 NETWORK_IS_SESSION_ADVERTISE
/// Script native `NETWORK_IS_SESSION_ADVERTISE` (hash 0x1B6716B8).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores the low byte of its answer (zero-extended) into the return
/// slot.
export!(cdecl, rw_00bd8920(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
