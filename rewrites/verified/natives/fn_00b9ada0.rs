// original: 0x00b9ada0 SWITCH_PED_PATHS_OFF

/// Native handler `SWITCH_PED_PATHS_OFF`.
///
/// Switch ped paths off inside a box given by six float bounds.
/// Forwards 6 argument(s) to the engine and returns nothing.
export!(cdecl, rw_00b9ada0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let arg2 = *args.add(2);
        let arg3 = *args.add(3);
        let arg4 = *args.add(4);
        let arg5 = *args.add(5);
        let _ = arg0;
        callee_cdecl!(1, u32, arg0, arg1, arg2, arg3, arg4, arg5);
        0
    }
});
