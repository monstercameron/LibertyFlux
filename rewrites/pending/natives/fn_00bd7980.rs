// original: 0x00bd7980 GET_EPISODE_INDEX_FROM_SUMMONS
/// Script native `GET_EPISODE_INDEX_FROM_SUMMONS` (hash 0x704E638F).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00bd7980(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
