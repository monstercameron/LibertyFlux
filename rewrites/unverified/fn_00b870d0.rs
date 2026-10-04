// original: 0x00b870d0 POINT_FIXED_CAM_AT_OBJ
/// Script native `POINT_FIXED_CAM_AT_OBJ` (hash 0x02326335).
///
/// Forwards two script arguments (a camera and an object handle) to the
/// engine. No return slot is written.
export!(cdecl, rw_00b870d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
