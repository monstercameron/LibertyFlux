// original: 0x009cc710 SET_TRAIN_AUDIO_ROLLOFF
/// Script native `SET_TRAIN_AUDIO_ROLLOFF` (hash 0x01C21158).
///
/// Forwards a train handle and a float bit-pattern to the engine worker.
/// No return slot is written.
export!(cdecl, rw_009cc710(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
