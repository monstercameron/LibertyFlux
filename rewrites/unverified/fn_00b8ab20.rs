// original: 0x00b8ab20 GET_CUTSCENE_TIME
/// Script native `GET_CUTSCENE_TIME` (hash 0x7DF26C8C).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// Stores the full 32-bit engine answer into the return slot.
export!(cdecl, rw_00b8ab20(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
