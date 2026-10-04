// original: 0x00bd9020 RESERVE_NETWORK_MISSION_OBJECTS_FOR_HOST
/// Script native `RESERVE_NETWORK_MISSION_OBJECTS_FOR_HOST` (hash 0x2F7508E7).
///
/// Forwards one script argument (an object count) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bd9020(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});

