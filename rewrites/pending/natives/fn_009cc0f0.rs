// original: 0x009cc0f0 PLAY_SOUND_FROM_OBJECT
/// Script native `PLAY_SOUND_FROM_OBJECT` (hash 0x60AE0867).
///
/// Forwards three script arguments (a sound id, an object handle and flags) to the sound-play engine. No return slot is written.
export!(cdecl, rw_009cc0f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
