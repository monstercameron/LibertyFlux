// original: 0x00b95110 SET_UP_TRIP_SKIP_FOR_VEHICLE_FINISHED_BY_SCRIPT
/// Script native `SET_UP_TRIP_SKIP_FOR_VEHICLE_FINISHED_BY_SCRIPT` (hash 0x4D5068A6).
///
/// Forwards 5 script arguments to the engine (raw bit patterns, so the forward is bit-exact). No return slot is written.
export!(cdecl, rw_00b95110(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4))
    }
});
