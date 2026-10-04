// original: 0x00b86a50 DO_SCREEN_FADE_OUT_UNHACKED
/// Script native handler `DO_SCREEN_FADE_OUT_UNHACKED` (hash 0x42D250A7).
///
/// Forwards script argument 0 to the engine worker and returns its answer.
export!(cdecl, rw_00b86a50(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let answer: u32 = callee_cdecl!(1, u32, *args);
        answer
    }
});
