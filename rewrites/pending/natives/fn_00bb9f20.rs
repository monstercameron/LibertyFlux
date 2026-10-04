// original: 0x00bb9f20 TASK_LEAVE_CAR_IMMEDIATELY
/// Script native `TASK_LEAVE_CAR_IMMEDIATELY` (hash 0x7BFB484F).
///
/// Forwards two script arguments (a character handle and a vehicle handle)
/// to the engine. No return slot is written.
export!(cdecl, rw_00bb9f20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
