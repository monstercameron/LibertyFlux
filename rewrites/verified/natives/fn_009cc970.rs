// original: 0x009cc970 UNFREEZE_RADIO_STATION
/// Script native `UNFREEZE_RADIO_STATION` (hash 0x3E5B7E59).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_009cc970(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
