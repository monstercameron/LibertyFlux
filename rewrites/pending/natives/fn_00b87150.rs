// original: 0x00b87150 POINT_FIXED_CAM_AT_VEHICLE
/// Script native `POINT_FIXED_CAM_AT_VEHICLE` (hash 0x52FF28DF).
///
/// Script arguments: ['veh', 'Car']: ?, ['cam', 'int']: ?.
///
/// Forwards arg0 (integer/handle), arg1 (integer/handle) to the engine routine.
/// No return slot is written.
export!(cdecl, rw_00b87150(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        callee_cdecl!(1, u32, arg0, arg1, )
    }
});
