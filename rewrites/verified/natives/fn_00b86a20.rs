// original: 0x00b86a20 DO_SCREEN_FADE_IN
/// Script native `DO_SCREEN_FADE_IN` (hash 0x04D72200).
///
/// Forwards one script argument (a fade duration) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b86a20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
