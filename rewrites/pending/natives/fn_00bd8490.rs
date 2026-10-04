// original: 0x00bd8490 NETWORK_GET_NEXT_TEXT_CHAT
/// Script native `NETWORK_GET_NEXT_TEXT_CHAT` (hash 0x314E106A).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores all 32 bits of its answer into the return slot.
export!(cdecl, rw_00bd8490(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32, ,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
