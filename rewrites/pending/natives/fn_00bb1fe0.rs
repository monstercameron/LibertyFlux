// original: 0x00bb1fe0 GET_PLAYER_TEAM
/// Script native handler `GET_PLAYER_TEAM`.
///
/// Reads the argument array at ctx+8, passes arg0 integer, calls the engine worker
/// (intercepted by the checker), and writes the engine answer to the return slot.
export!(cdecl, rw_00bb1fe0(ctx: *const u8) -> u32 {
    unsafe {
        let args = *(ctx.add(8) as *const *const u32);
        let a0 = *args.add(0);
        let ans: u32 = callee_cdecl!(1, u32, a0,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = ans;
        ans
    }
});
