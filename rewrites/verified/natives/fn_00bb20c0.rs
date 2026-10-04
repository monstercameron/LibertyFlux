// original: 0x00bb20c0 GET_TIME_SINCE_PLAYER_HIT_CAR
/// Script native `GET_TIME_SINCE_PLAYER_HIT_CAR` (hash 0x58C01823).
///
/// Forwards one script argument (a player index) to the engine and stores its full 32-bit answer into the return slot. Unlike the boolean natives, this handler keeps the whole answer (`mov`, not `movzx`).
export!(cdecl, rw_00bb20c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
