// original: 0x00bd9040 RESERVE_NETWORK_MISSION_PEDS_FOR_HOST
/// Script native `RESERVE_NETWORK_MISSION_PEDS_FOR_HOST` (hash 0x557C7C4A).
///
/// Forwards one script word (a count) to the engine. No return slot
/// is written.
export!(cdecl, rw_00bd9040(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
