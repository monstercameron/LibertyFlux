// original: 0x00bba740 TASK_SHOOT_AT_COORD
/// Script native `TASK_SHOOT_AT_COORD` (hash 0x705231A9).
///
/// Forwards arg0, arg1 (float bits), arg2 (float bits), arg3 (float bits), arg4, arg5 to the engine.
/// No return slot is written.
export!(cdecl, rw_00bba740(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5));
        ans
    }
});
