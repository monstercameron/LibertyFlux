// original: 0x00bb8ef0 SET_DRIVE_TASK_CRUISE_SPEED
/// Set a drive task's cruise speed: pass the task handle plus the speed
/// float (bitwise) to the engine. No return value.
export!(cdecl, rw_00bb8ef0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1));
        0
    }
});
