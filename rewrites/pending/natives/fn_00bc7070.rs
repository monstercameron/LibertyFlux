// original: 0x00bc7070 MARK_MISSION_TRAIN_AS_NO_LONGER_NEEDED
/// Script native `MARK_MISSION_TRAIN_AS_NO_LONGER_NEEDED` (hash 0x37AC2A95).
///
/// Forwards one script argument (a vehicle handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bc7070(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
