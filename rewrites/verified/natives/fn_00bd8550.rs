// original: 0x00bd8550 NETWORK_GET_UNACCEPTED_INVITER_NAME
/// Script native `NETWORK_GET_UNACCEPTED_INVITER_NAME` (hash 0x1A7B3125).
///
/// Forwards one script argument to the engine and stores the engine's full
/// 32-bit answer into the return slot.
export!(cdecl, rw_00bd8550(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
