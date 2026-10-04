// original: 0x00b87cc0 SET_TELESCOPE_CAM_ANGLE_LIMITS
/// Script native `SET_TELESCOPE_CAM_ANGLE_LIMITS` (hash 0x6680196B).
///
/// Forwards six float script arguments to the engine as raw bits, so the forward is bit-exact.
export!(cdecl, rw_00b87cc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
        )
    }
});
