// original: 0x00bc6000 GET_NUMBER_OF_PASSENGERS
/// Script native handler `GET_NUMBER_OF_PASSENGERS`.
///
/// Reads the argument array at ctx+8, passes arg0 integer, arg1 integer, calls the engine worker
/// (intercepted by the checker), and returns nothing to the script (the engine answer stays in EAX).
export!(cdecl, rw_00bc6000(ctx: *const u8) -> u32 {
    unsafe {
        let args = *(ctx.add(8) as *const *const u32);
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let ans: u32 = callee_cdecl!(1, u32, a0, a1,);
        ans
    }
});
