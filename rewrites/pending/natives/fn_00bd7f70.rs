// original: 0x00bd7f70 IS_PARTY_MODE
/// Script native `IS_PARTY_MODE` (hash 0x2A3A77FD).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd7f70(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
