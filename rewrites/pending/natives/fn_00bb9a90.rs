// original: 0x00bb9a90 TASK_GO_STRAIGHT_TO_COORD_RELATIVE_TO_CAR
/// Native handler `TASK_GO_STRAIGHT_TO_COORD_RELATIVE_TO_CAR`.
///
/// Task a ped to go straight to car-relative coordinates.
/// Forwards 7 argument(s) to the engine and returns nothing.
export!(cdecl, rw_00bb9a90(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let arg2 = *args.add(2);
        let arg3 = *args.add(3);
        let arg4 = *args.add(4);
        let arg5 = *args.add(5);
        let arg6 = *args.add(6);
        let _ = arg0;
        callee_cdecl!(1, u32, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
        0
    }
});
