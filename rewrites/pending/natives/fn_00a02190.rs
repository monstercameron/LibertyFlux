// original: 0x00a02190 SET_WEAPON_PICKUP_NETWORK_REGEN_TIME
/// Script native `SET_WEAPON_PICKUP_NETWORK_REGEN_TIME` (hash 0x40D01439).
///
/// Forwards 2 script arguments (a pickup handle and a time value) to the engine.
/// No return slot is written.
export!(cdecl, rw_00a02190(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
