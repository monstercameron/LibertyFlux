// original: 0x00bc6510 GET_VEHICLE_QUATERNION
/// Script native `GET_VEHICLE_QUATERNION` (hash 0x6C5871D6).
///
/// Forwards five script arguments (a vehicle handle and four out-pointers) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bc6510(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4),)
    }
});
