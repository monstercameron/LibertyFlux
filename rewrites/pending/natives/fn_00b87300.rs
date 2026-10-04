// original: 0x00b87300 SET_CAM_DOF_FOCUSPOINT
/// Script native `SET_CAM_DOF_FOCUSPOINT` (hash 0x39DC5AEB).
///
/// Forwards five script arguments to the engine: an integer and four float
/// bit-patterns (depth-of-field focus parameters). No return slot is
/// written.
export!(cdecl, rw_00b87300(ctx: *const u8) -> u32 {
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
        )
    }
});
