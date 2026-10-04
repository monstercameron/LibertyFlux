// original: 0x00b94e50 SET_FAKE_WANTED_CIRCLE
/// Script native handler `SET_FAKE_WANTED_CIRCLE`.
///
/// Reads the argument array at ctx+8, passes arg0 float bits, arg1 float bits, arg2 float bits, calls the engine worker
/// (intercepted by the checker), and returns nothing to the script (the engine answer stays in EAX).
export!(cdecl, rw_00b94e50(ctx: *const u8) -> u32 {
    unsafe {
        let args = *(ctx.add(8) as *const *const u32);
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let ans: u32 = callee_cdecl!(1, u32, a0, a1, a2,);
        ans
    }
});
