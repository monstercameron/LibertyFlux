// original: 0x009cc900 TRIGGER_MISSION_COMPLETE_AUDIO
/// Script native `TRIGGER_MISSION_COMPLETE_AUDIO` (hash 0x4BAF0213).
///
/// Forwards one script argument to the engine.
export!(cdecl, rw_009cc900(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
