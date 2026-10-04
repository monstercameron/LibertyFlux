// original: 0x009cc040 PLAY_AUDIO_EVENT_FROM_VEHICLE
/// Script native `PLAY_AUDIO_EVENT_FROM_VEHICLE` (hash 0x2F4B2A8B).
///
/// Forwards 2 script arguments (an audio event hash and a vehicle handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_009cc040(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});

