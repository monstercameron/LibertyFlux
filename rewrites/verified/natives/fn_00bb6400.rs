// original: 0x00bb6400 GET_TOTAL_NUMBER_OF_STATS
/// Script native `GET_TOTAL_NUMBER_OF_STATS` (hash 0x6D823703).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the full 32-bit answer into the return slot.
export!(cdecl, rw_00bb6400(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
