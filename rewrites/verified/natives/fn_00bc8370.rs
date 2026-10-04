// original: 0x00bc8370 UNPAUSE_PLAYBACK_RECORDED_CAR
/// Script native `UNPAUSE_PLAYBACK_RECORDED_CAR` (hash 0x361A01AD).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00bc8370(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
