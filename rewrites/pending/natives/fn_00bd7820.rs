// original: 0x00bd7820 DOES_GAME_CODE_WANT_TO_LEAVE_NETWORK_SESSION
/// Script native `DOES_GAME_CODE_WANT_TO_LEAVE_NETWORK_SESSION`
/// (hash 0x7E412AC8).
///
/// Takes no script arguments: calls the engine predicate with no arguments
/// and stores the low byte of its answer (zero-extended) into the return
/// slot.
export!(cdecl, rw_00bd7820(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
