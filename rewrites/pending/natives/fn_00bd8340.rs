// original: 0x00bd8340 NETWORK_GET_FRIEND_NAME
/// Script native `NETWORK_GET_FRIEND_NAME` (hash 0x17FD0934).
///
/// Forwards one script argument (a friend index) to the engine and stores the
/// full 32-bit answer into the return slot.
export!(cdecl, rw_00bd8340(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
