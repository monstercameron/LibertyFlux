// original: 0x00b866c0 ATTACH_CAM_TO_VEHICLE
/// Script native `ATTACH_CAM_TO_VEHICLE` (hash 0x5E564CFF).
///
/// Forwards two script arguments (a vehicle handle and a camera handle) to the engine.
///
/// No return slot is written.
export!(cdecl, rw_00b866c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
