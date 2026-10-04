// original: 0x00BB9F40 TASK_LEAVE_CAR_IN_DIRECTION
// TASK_LEAVE_CAR_IN_DIRECTION: forward (ped, vehicle, direction?) with the
// third argument coerced through the incoming stack slot.
export!(cdecl, rw_00BB9F40(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1), coerced(ctx, *a.add(2)))
    }
});
