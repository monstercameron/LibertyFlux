// original: 0x00b86a40 DO_SCREEN_FADE_OUT
// Rewrite of the DO_SCREEN_FADE_OUT native handler.

/// Script native `DO_SCREEN_FADE_OUT(duration)`.
///
/// Forwards the fade duration to the engine fade routine. No return slot is
/// written; the engine's answer is left in the return register.
export!(cdecl, rw_b86a40(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        callee_cdecl!(1, u32, *args)
    }
});
