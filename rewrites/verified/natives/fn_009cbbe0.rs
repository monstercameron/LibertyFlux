// original: 0x009cbbe0 GET_CURRENT_SCRIPTED_CONVERSATION_LINE
/// Script native `GET_CURRENT_SCRIPTED_CONVERSATION_LINE` (hash 0x0DE30821).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer into the return slot.
export!(cdecl, rw_009cbbe0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
