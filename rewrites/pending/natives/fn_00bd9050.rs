// original: 0x00bd9050 RESERVE_NETWORK_MISSION_VEHICLES
/// Native handler `RESERVE_NETWORK_MISSION_VEHICLES`.
///
/// Reserve mission vehicles for the network session.
/// Forwards 1 argument(s) to the engine and returns nothing.
export!(cdecl, rw_00bd9050(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let _ = arg0;
        callee_cdecl!(1, u32, arg0);
        0
    }
});
