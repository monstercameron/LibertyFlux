// original: 0x00bc8020 SKIP_TIME_IN_PLAYBACK_RECORDED_CAR
/// Script native `SKIP_TIME_IN_PLAYBACK_RECORDED_CAR` (hash 0x255059BB).
///
/// Forwards two script arguments to the engine: an integer (a vehicle
/// handle) and one float bit-pattern (a time skip). The float is copied as
/// raw bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bc8020(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
