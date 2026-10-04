// original: 0x00b86a30 DO_SCREEN_FADE_IN_UNHACKED
/// Script native `DO_SCREEN_FADE_IN_UNHACKED` (hash 0x5F9218C3).
///
/// Forwards one script argument (the fade duration) to the engine. No
/// return slot is written.
export!(cdecl, rw_00b86a30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
