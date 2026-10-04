// original: 0x00bb6260 AWARD_PLAYER_MISSION_RESPECT
/// Script native `AWARD_PLAYER_MISSION_RESPECT` (hash 0x7783449D).
///
/// Forwards one script argument (a float bit-pattern, moved through an SSE
/// register) to the engine. No return slot is written.
export!(cdecl, rw_00bb6260(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
