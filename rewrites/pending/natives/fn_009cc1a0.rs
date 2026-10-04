// original: 0x009cc1a0 PLAY_SOUND_FRONTEND
/// Script native `PLAY_SOUND_FRONTEND` (hash 0x4DAF2C87).
///
/// Forwards two script arguments (sound and entity identifiers) to the
/// engine. No return slot is written.
export!(cdecl, rw_009cc1a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
