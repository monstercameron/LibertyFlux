// original: 0x00bb2100 GET_TIME_SINCE_PLAYER_HIT_PED
// GET_TIME_SINCE_PLAYER_HIT_PED: forwards the player index and stores the
// engine's full dword answer into the return slot.
export!(cdecl, rw_fn_bb2100(ctx: *mut u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let ans: u32 = callee_cdecl!(1, u32, *args);
        let ret = *(ctx as *mut *mut u32);
        *ret = ans;
        ans
    }
});
