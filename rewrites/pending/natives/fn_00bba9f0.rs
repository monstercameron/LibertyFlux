// original: 0x00bba9f0 TASK_SMART_FLEE_POINT
/// Script native `TASK_SMART_FLEE_POINT` (hash 0x7381337A).
///
/// Forwards six script arguments (a ped handle and five float/int words) to the engine. No return slot is written.
export!(cdecl, rw_00bba9f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5))
    }
});
