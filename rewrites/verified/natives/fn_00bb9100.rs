// original: 0x00bb9100 TASK_CAR_DRIVE_TO_COORD_NOT_AGAINST_TRAFFIC
/// Script native `TASK_CAR_DRIVE_TO_COORD_NOT_AGAINST_TRAFFIC` (hash 0x483A62AB).
///
/// Forwards eleven script arguments to the engine: two handles, five float
/// bit-patterns (target coordinates and cruise values) and four integer
/// options. Floats are copied as raw bits. No return slot is written.
export!(cdecl, rw_00bb9100(ctx: *const u8) -> u32 {
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
            *args.add(6),
            *args.add(7),
            *args.add(8),
            *args.add(9),
            *args.add(10),
        )
    }
});
