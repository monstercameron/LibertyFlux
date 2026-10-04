// original: 0x009cc4c0 SET_GPS_VOICE_FOR_VEHICLE
/// Script native `SET_GPS_VOICE_FOR_VEHICLE` (hash 0x356876BF).
///
/// Forwards two script arguments (a vehicle handle and a voice id) to the
/// engine. No return slot is written.
export!(cdecl, rw_009cc4c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
