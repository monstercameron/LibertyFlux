// original: 0x00b86c30 GET_FOLLOW_VEHICLE_CAM_SUBMODE
/// Script native `GET_FOLLOW_VEHICLE_CAM_SUBMODE` (hash 0x4C7B7A29).
///
/// Forwards one script argument (an out-pointer) to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b86c30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
