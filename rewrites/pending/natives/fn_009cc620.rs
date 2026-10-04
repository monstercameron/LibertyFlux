// original: 0x009cc620 SET_ROMANS_MOOD
/// Script native `SET_ROMANS_MOOD` (hash 0x126F1175).
///
/// Forwards one script argument (a mood value) to the engine. No return
/// slot is written.
export!(cdecl, rw_009cc620(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
