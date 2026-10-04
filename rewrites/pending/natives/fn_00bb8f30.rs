// original: 0x00bb8f30 SET_GUNSHOT_SENSE_RANGE_FOR_RIOT2
/// Script native handler `SET_GUNSHOT_SENSE_RANGE_FOR_RIOT2`.
///
/// Reads the argument array at ctx+8, passes arg0 float bits, calls the engine worker
/// (intercepted by the checker), and returns nothing to the script (the engine answer stays in EAX).
export!(cdecl, rw_00bb8f30(ctx: *const u8) -> u32 {
    unsafe {
        let args = *(ctx.add(8) as *const *const u32);
        let a0 = *args.add(0);
        let ans: u32 = callee_cdecl!(1, u32, a0,);
        ans
    }
});
