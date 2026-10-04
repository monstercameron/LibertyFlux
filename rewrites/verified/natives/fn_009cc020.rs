// original: 0x009cc020 PLAY_AUDIO_EVENT_FROM_PED
/// Script native `PLAY_AUDIO_EVENT_FROM_PED` (hash 0x61064783).
///
/// Forwards two script arguments (an audio event hash and a ped handle) to
/// the engine. No return slot is written.
export!(cdecl, rw_009cc020(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
