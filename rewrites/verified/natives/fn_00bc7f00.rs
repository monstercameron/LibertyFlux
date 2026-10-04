// original: 0x00bc7f00 SET_VEHICLE_STEER_BIAS
/// Script native `SET_VEHICLE_STEER_BIAS` (hash 0x091D1480).
///
/// Forwards two script arguments to the engine: a vehicle handle and one
/// float bit-pattern (the steer bias), copied as raw bits. No return slot
/// is written.
export!(cdecl, rw_00bc7f00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
