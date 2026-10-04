// original: 0x009cc500 SET_LOCAL_PLAYER_PAIN_VOICE
/// Script native `SET_LOCAL_PLAYER_PAIN_VOICE` (hash 0x1DDD0073).
///
/// Forwards one script argument (a voice index) to the engine. No return slot
/// is written.
export!(cdecl, rw_009cc500(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
