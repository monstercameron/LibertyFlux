// original: 0x00b87070 POINT_CAM_AT_VEHICLE
/// Script native `POINT_CAM_AT_VEHICLE` (hash 0x69F02BA0).
///
/// Camera + vehicle handles.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00b87070(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
