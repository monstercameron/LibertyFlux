// original: 0x00bb1dd0 GET_LEFT_PLAYER_CASH_TO_REACH_LEVEL
/// Script native `GET_LEFT_PLAYER_CASH_TO_REACH_LEVEL` (hash 0x6DD754DD).
///
/// Forwards one script argument to the engine and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00bb1dd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
