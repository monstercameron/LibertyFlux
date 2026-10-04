// original: 0x00b87090 POINT_FIXED_CAM
/// Script native `POINT_FIXED_CAM` (hash 0x04FF3F49).
///
/// Forwards four script arguments to the engine: three float bit-patterns
/// (a target position) and one integer (a duration or flag). The floats
/// travel as raw bits and are bit-exact by construction.
/// No return slot is written.
export!(cdecl, rw_00b87090(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
