// original: 0x00a01a20 SET_ARMOUR_PICKUP_NETWORK_REGEN_TIME
/// Script native `SET_ARMOUR_PICKUP_NETWORK_REGEN_TIME` (hash 0x53CC1D3C).
///
/// Forwards one integer (the respawn time) to the engine. No return slot is
/// written.
export!(cdecl, rw_00a01a20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
        )
    }
});
