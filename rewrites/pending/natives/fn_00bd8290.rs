// original: 0x00bd8290 NETWORK_EXPAND_TO_32_PLAYERS
/// Script native `NETWORK_EXPAND_TO_32_PLAYERS` (hash 0x36511E0A).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd8290(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
