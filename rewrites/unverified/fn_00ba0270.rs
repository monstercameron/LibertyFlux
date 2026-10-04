// original: 0x00ba0270 IS_PLAYER_BEING_ARRESTED
/// Script native `IS_PLAYER_BEING_ARRESTED` (hash 0x79A95BF9).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores the low byte of its answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00ba0270(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32, );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
