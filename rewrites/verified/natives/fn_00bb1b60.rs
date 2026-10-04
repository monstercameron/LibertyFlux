// original: 0x00bb1b60 BLOCK_STATS_MENU_ACTIONS
/// Script native `BLOCK_STATS_MENU_ACTIONS` (hash 0x734E3F62).
///
/// Forwards one script argument (a boolean flag) to the engine as a word.
/// No return slot is written; the engine answer is the exit value.
export!(cdecl, rw_00bb1b60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
