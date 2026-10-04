// original: 0x00b870f0 POINT_FIXED_CAM_AT_PED
// Rewrite of the POINT_FIXED_CAM_AT_PED native handler.

/// Script native `POINT_FIXED_CAM_AT_PED(cam, ped)`.
///
/// Forwards the two script arguments to the engine camera-point routine. No
/// return slot is written; the engine's answer is left in the return register.
export!(cdecl, rw_b870f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
