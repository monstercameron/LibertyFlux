// original: 0x00bc70c0 PAUSE_PLAYBACK_RECORDED_CAR
/// Script native `PAUSE_PLAYBACK_RECORDED_CAR` (hash 0x24256EFB).
///
/// Forwards one script argument (a vehicle handle) to the engine. No return
/// slot is written; the engine answer is the exit value.
export!(cdecl, rw_00bc70c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
