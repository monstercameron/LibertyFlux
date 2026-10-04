// original: 0x00bb20e0 GET_TIME_SINCE_PLAYER_HIT_OBJECT
/// Script native `GET_TIME_SINCE_PLAYER_HIT_OBJECT` (hash 0x43C2796B).
///
/// Forwards one script argument (a player handle) to the engine and
/// stores the engine's full 32-bit answer into the return slot.
export!(cdecl, rw_00bb20e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
