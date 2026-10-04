// original: 0x00a01970 RESET_WEAPON_PICKUP_NETWORK_REGEN_TIME
/// Script native `RESET_WEAPON_PICKUP_NETWORK_REGEN_TIME` (hash 0x5F3459B2).
///
/// Forwards one script argument (a weapon pickup handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00a01970(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
