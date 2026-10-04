// original: 0x00b943a0 GET_CURRENT_EPISODE
/// Script native `GET_CURRENT_EPISODE` (hash 0x7D7619D2).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores all 32 bits of its answer into the return slot.
export!(cdecl, rw_00b943a0(ctx: *const u8) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
