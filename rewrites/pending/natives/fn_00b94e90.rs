// original: 0x00B94E90 SET_FAKE_WANTED_LEVEL
/// Script native handler `SET_FAKE_WANTED_LEVEL`.
///
/// Reads the argument array at ctx+8, passes arg0 integer, calls the engine worker
/// (intercepted by the checker), and returns nothing to the script (the engine answer stays in EAX).
export!(cdecl, rw_00b94e90(ctx: *const u8) -> u32 {
    unsafe {
        let args = *(ctx.add(8) as *const *const u32);
        let a0 = *args.add(0);
        let ans: u32 = callee_cdecl!(1, u32, a0,);
        ans
    }
});
