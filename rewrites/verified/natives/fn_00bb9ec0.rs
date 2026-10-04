// original: 0x00bb9ec0 TASK_LEAVE_CAR_AND_FLEE
/// Script native `TASK_LEAVE_CAR_AND_FLEE` (hash 0x6CEA50D8).
///
/// Forwards five script arguments to the engine: a character handle, a
/// vehicle handle and three float bit-patterns (a flee target position).
/// Floats are copied as raw bits. No return slot is written.
export!(cdecl, rw_00bb9ec0(ctx: *const u8) -> u32 {
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
