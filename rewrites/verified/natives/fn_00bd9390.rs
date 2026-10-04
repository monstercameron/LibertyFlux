// original: 0x00bd9390 SET_NETWORK_PED_USING_PARACHUTE
/// Script native `SET_NETWORK_PED_USING_PARACHUTE` (hash 0x6E8B7611).
///
/// Forwards one script argument (a network ped handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bd9390(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
