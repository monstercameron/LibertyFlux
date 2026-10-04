// original: 0x00bba780 TASK_SHUFFLE_TO_NEXT_CAR_SEAT
// rw_task_shuffle_to_next_car_seat: native TASK_SHUFFLE_TO_NEXT_CAR_SEAT (handler 0x00BBA780).
//
// Forwards a ped handle and a car handle to the seat-shuffle task assigner. No return slot.
export!(cdecl, rw_task_shuffle_to_next_car_seat(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1));
        ans
    }
});
