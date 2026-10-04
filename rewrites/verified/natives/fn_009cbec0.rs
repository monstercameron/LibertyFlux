// original: 0x009cbec0 IS_SCRIPTED_CONVERSATION_ONGOING
// rw_is_scripted_conversation_ongoing: native IS_SCRIPTED_CONVERSATION_ONGOING (handler 0x009CBEC0).
//
// No-arg query: calls the conversation-state check, stores the low byte of the answer in the return slot.
export!(cdecl, rw_is_scripted_conversation_ongoing(ctx: *mut u32) -> u32 {
    unsafe {
        let ans = callee_cdecl!(1, u32,);
        *(*ctx as *mut u32) = ans & 0xFF;
        ans
    }
});
