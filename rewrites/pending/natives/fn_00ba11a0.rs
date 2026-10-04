// original: 0x00BA11A0 SET_CHAR_COORDINATES
/// Script native handler `SET_CHAR_COORDINATES`.
///
/// Reads the argument array at ctx+8, passes arg0 integer, arg1 float bits, arg2 float bits, arg3 float bits, calls the engine worker
/// (intercepted by the checker), and returns nothing to the script (the engine answer stays in EAX).
export!(cdecl, rw_00ba11a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = *(ctx.add(8) as *const *const u32);
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        let ans: u32 = callee_cdecl!(1, u32, a0, a1, a2, a3,);
        ans
    }
});
