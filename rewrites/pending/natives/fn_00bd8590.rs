// original: 0x00bd8590 NETWORK_GET_UNACCEPTED_INVITE_GAME_MODE
/// Script native `NETWORK_GET_UNACCEPTED_INVITE_GAME_MODE` (hash 0x5E44065D).
///
/// Forwards one script argument (a player index) to the engine and stores
/// its full 32-bit answer into the return slot.
export!(cdecl, rw_00bd8590(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
