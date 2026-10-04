// original: 0x00bd7c00 GET_PLAYER_RANK_LEVEL_DURING_MP
/// Script native `GET_PLAYER_RANK_LEVEL_DURING_MP` (hash 0x7B31633E).
///
/// Forwards one script argument (a player index) to the engine and stores
/// the full 32-bit engine answer into the return slot. The handler leaves
/// the engine answer in EAX (it addresses the slot through ECX), so the
/// exit value is the answer, not the slot pointer.
export!(cdecl, rw_00bd7c00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
