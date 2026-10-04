// original: 0x00bb1e80 GET_NUMBER_OF_PLAYERS
/// Script native `GET_NUMBER_OF_PLAYERS` (hash 0x62405882).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores its full 32-bit answer into the return slot. Unlike the
/// boolean natives, this handler keeps the whole answer (`mov`, not
/// `movzx`).
export!(cdecl, rw_00bb1e80(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
