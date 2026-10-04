// original: 0x00bd8e00 NETWORK_SHOW_PLAYER_FEEDBACK_UI
/// Script native `NETWORK_SHOW_PLAYER_FEEDBACK_UI` (hash 0x6FC54C6B).
///
/// Forwards one script argument (a player handle) to the engine. No
/// return slot is written.
export!(cdecl, rw_00bd8e00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
