// original: 0x00bd7e50 IS_NETWORK_CONNECTED
/// Script native handler `IS_NETWORK_CONNECTED` (hash 0x43945A83).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd7e50(ctx: u32) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
