// original: 0x00bb2e30 WHAT_WILL_PLAYER_PICKUP
/// Script native `WHAT_WILL_PLAYER_PICKUP` (hash 0x2F9B0583).
///
/// Forwards one script argument (a player index) to the engine and stores
/// its full 32-bit answer into the return slot. Unlike the boolean natives,
/// this handler keeps the whole answer (`mov`, not `movzx`).
export!(cdecl, rw_00bb2e30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
