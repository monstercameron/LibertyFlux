// original: 0x00b87030 POINT_CAM_AT_OBJECT
/// Script native `POINT_CAM_AT_OBJECT` (hash 0x5E627D20).
///
/// Forwards two script arguments (a camera handle and an object handle)
/// to the engine.
/// No return slot is written.
export!(cdecl, rw_00b87030(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
