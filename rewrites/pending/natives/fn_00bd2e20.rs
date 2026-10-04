// original: 0x00bd2e20 SET_ZONE_SCUMMINESS
/// Native handler `SET_ZONE_SCUMMINESS`: forwards script args [arg0 (dword), arg1 (dword)]
/// to its engine function and returns nothing (void).
export!(cdecl, rw_00bd2e20(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args.add(0), *args.add(1));
        0
    }
});
