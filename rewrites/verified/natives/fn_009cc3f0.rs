// original: 0x009cc3f0 RETUNE_RADIO_TO_STATION_NAME
/// Script native `RETUNE_RADIO_TO_STATION_NAME` (hash 0x58BA4401).
///
/// Forwards one script argument (a station-name string) to the engine.
/// No return slot is written.
export!(cdecl, rw_009cc3f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
