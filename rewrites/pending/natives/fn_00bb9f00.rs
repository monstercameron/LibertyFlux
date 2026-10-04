// original: 0x00BB9F00 TASK_LEAVE_CAR_DONT_CLOSE_DOOR
// TASK_LEAVE_CAR_DONT_CLOSE_DOOR: forward (ped, vehicle) to the engine.
export!(cdecl, rw_00BB9F00(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1))
    }
});
