// original: 0x009cbff0 PLAY_AUDIO_EVENT
/// Script native `PLAY_AUDIO_EVENT` (hash 0x486F3D93).
///
/// Forwards one script argument (an audio event id) to the engine. No
/// return slot is written.
export!(cdecl, rw_009cbff0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
