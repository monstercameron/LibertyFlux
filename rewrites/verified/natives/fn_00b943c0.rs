// original: 0x00b943c0 GET_CURRENT_STACK_SIZE
/// Script native `GET_CURRENT_STACK_SIZE` (hash 0x6AC52840).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// Stores the full 32-bit engine answer into the return slot.
export!(cdecl, rw_00b943c0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
