// original: 0x009cc9a0 UNLOCK_MISSION_NEWS_STORY
/// Script native `UNLOCK_MISSION_NEWS_STORY` (hash 0x2F0718CA).
///
/// Forwards one script argument (a story id) to the engine. No return slot
/// is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_009cc9a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
