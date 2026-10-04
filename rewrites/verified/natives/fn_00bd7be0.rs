// original: 0x00bd7be0 GET_PLAYER_LCPD_SCORE
/// Script native `GET_PLAYER_LCPD_SCORE` (hash 0x2BEB02D6).
///
/// Forwards one script argument (a player index) to the engine and stores the
/// full 32-bit answer into the return slot. Unlike the `movzx` shapes, this
/// handler leaves the engine answer in EAX on exit, so the rewrite returns it.
export!(cdecl, rw_00bd7be0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
