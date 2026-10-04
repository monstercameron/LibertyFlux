// original: 0x00B8C450 DISPLAY_TEXT_WITH_TWO_SUBSTRINGS_GIVEN_HASH_KEYS
/// Script native handler `DISPLAY_TEXT_WITH_TWO_SUBSTRINGS_GIVEN_HASH_KEYS`.
///
/// Reads the argument array at ctx+8, passes arg0 float bits, arg1 float bits, arg2 integer, arg3 integer, arg4 integer, calls the engine worker
/// (intercepted by the checker), and returns nothing to the script (the engine answer stays in EAX).
export!(cdecl, rw_00b8c450(ctx: *const u8) -> u32 {
    unsafe {
        let args = *(ctx.add(8) as *const *const u32);
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        let a4 = *args.add(4);
        let ans: u32 = callee_cdecl!(1, u32, a0, a1, a2, a3, a4,);
        ans
    }
});
