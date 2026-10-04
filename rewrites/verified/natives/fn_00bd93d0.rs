// original: 0x00bd93d0 SET_NETWORK_VEHICLE_RESPOT_TIMER
/// Script native `SET_NETWORK_VEHICLE_RESPOT_TIMER` (hash 0x266F327C).
///
/// Forwards two script arguments (a vehicle handle and a timer value) to
/// the engine. No return slot is written.
export!(cdecl, rw_00bd93d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
