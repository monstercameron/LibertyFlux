// original: 0x009cc370 REPORT_TAGGED_RADIO_TRACK
/// Script native `REPORT_TAGGED_RADIO_TRACK` (hash 0x0ED8621F).
///
/// Forwards one script argument (a track id) to the engine.
/// No return slot is written.
export!(cdecl, rw_009cc370(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
