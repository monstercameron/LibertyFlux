// original: 0x009cc980 UNLOCK_GENERIC_NEWS_STORY
/// Script native `UNLOCK_GENERIC_NEWS_STORY` (hash 0x06BE0DD3).
///
/// Forwards one script argument (a news-story index) to the engine.
/// No return slot is written.
export!(cdecl, rw_009cc980(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
