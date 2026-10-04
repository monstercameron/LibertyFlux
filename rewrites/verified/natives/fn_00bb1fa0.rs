// original: 0x00bb1fa0 GET_PLAYER_NAME
/// Script native `GET_PLAYER_NAME` (hash 0x570F5725).
///
/// Forwards one script argument (a player index) to the engine and stores
/// its full 32-bit answer (a string pointer) into the return slot.
export!(cdecl, rw_00bb1fa0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
