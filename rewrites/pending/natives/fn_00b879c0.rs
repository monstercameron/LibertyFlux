// original: 0x00b879c0 SET_FOLLOW_VEHICLE_CAM_SUBMODE
/// Script native `SET_FOLLOW_VEHICLE_CAM_SUBMODE` (hash 0x20BC708E).
///
/// Forwards one script argument (a camera submode) to the engine. No return slot is written.
export!(cdecl, rw_00b879c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
