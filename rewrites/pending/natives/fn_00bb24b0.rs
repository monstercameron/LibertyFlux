// original: 0x00bb24b0 IS_PLAYER_IN_SHORTCUT_TAXI
/// Script native `IS_PLAYER_IN_SHORTCUT_TAXI` (hash 0x44052D59).
///
/// Forwards one script argument (a player index) to the engine taxi query
/// and stores the low byte of the answer (a boolean) into the return slot.
export!(cdecl, rw_00bb24b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(1, u32, *args);
        *slot = answer & 0xFF;
        slot as u32
    }
});
