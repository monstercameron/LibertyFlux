// original: 0x00b87440 SET_CAM_INTERP_STYLE_CORE
/// Script native `SET_CAM_INTERP_STYLE_CORE` (hash 0x72297CDC).
///
/// Forwards five script arguments (camera handles and style values) to
/// the engine.
/// No return slot is written.
export!(cdecl, rw_00b87440(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        );
        answer
    }
});
