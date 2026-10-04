// original: 0x00bd8410 NETWORK_GET_HOST_SERVER_NAME
/// Script native `NETWORK_GET_HOST_SERVER_NAME` (hash 0x031D740F).
///
/// Forwards one script argument (a player index) to the engine.
/// Stores the full 32-bit engine answer into the return slot.
export!(cdecl, rw_00bd8410(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
