// original: 0x00bc64b0 GET_VEHICLE_DIRT_LEVEL
/// Script native `GET_VEHICLE_DIRT_LEVEL` (hash 0x571152F5).
///
/// Forwards 2 script arguments (a vehicle handle and an out-pointer) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bc64b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});

