// original: 0x009cc000 PLAY_AUDIO_EVENT_FROM_OBJECT
/// Script native `PLAY_AUDIO_EVENT_FROM_OBJECT` (hash 0x4BB9178A).
///
/// Forwards two script arguments (an audio event hash and an object handle)
/// to the engine. No return slot is written.
export!(cdecl, rw_009cc000(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
