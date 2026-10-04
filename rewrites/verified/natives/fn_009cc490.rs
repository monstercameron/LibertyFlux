// original: 0x009cc490 SET_AMBIENT_VOICE_NAME
/// Script native `SET_AMBIENT_VOICE_NAME` (hash 0x426A4ED8).
///
/// Forwards two script arguments (a character handle and a voice-name string id) to the engine. No return slot is written.
export!(cdecl, rw_009cc490(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
