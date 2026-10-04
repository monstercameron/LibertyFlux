// original: 0x00bb9ea0 TASK_LEAVE_CAR
/// Native handler `TASK_LEAVE_CAR`: forwards script args [arg0 (dword), arg1 (dword)]
/// to its engine function and returns nothing (void).
export!(cdecl, rw_00bb9ea0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args.add(0), *args.add(1));
        0
    }
});
