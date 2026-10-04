// original: 0x00bb63c0 GET_STAT_FRONTEND_DISPLAY_TYPE
/// Script native `GET_STAT_FRONTEND_DISPLAY_TYPE` (hash 0x347C4300).
///
/// Forwards a stat id to the engine and stores the full 32-bit engine
/// answer (the frontend display type) into the return slot.
export!(cdecl, rw_00bb63c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
