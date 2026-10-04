// original: 0x00b87250 SET_CAM_ATTACH_OFFSET
/// Native handler `SET_CAM_ATTACH_OFFSET`: forwards script args [arg0 (dword), arg1 (float bits), arg2 (float bits), arg3 (float bits)]
/// to its engine function and returns nothing (void).
export!(cdecl, rw_00b87250(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3));
        0
    }
});
