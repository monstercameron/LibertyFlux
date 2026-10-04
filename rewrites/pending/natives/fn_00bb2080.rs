// original: 0x00bb2080 GET_TIME_SINCE_PLAYER_DROVE_ON_PAVEMENT
/// Script native `GET_TIME_SINCE_PLAYER_DROVE_ON_PAVEMENT` (hash 0x19610E35).
///
/// Forwards one script argument (a player index) to the engine and
/// stores its full 32-bit answer (the elapsed time) into the return slot.
export!(cdecl, rw_00bb2080(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});

