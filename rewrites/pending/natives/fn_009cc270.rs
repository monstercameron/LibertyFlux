// original: 0x009cc270 RELEASE_SOUND_ID
/// Script native `RELEASE_SOUND_ID` (hash 0x211D390A).
///
/// Forwards one script argument (a sound id) to the engine.
/// No return slot is written.
export!(cdecl, rw_009cc270(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
