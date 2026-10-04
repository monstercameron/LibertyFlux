// original: 0x00bb1e60 GET_NO_OF_PLAYERS_IN_TEAM
/// Script native `GET_NO_OF_PLAYERS_IN_TEAM` (hash 0x1CFD32E5).
///
/// Forwards one script argument (a team index) to the engine and
/// stores its full 32-bit answer (the player count) into the return slot.
export!(cdecl, rw_00bb1e60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});

