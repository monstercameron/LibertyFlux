// original: 0x00bb2050 GET_TIME_SINCE_LAST_DEATH
// rw_get_time_since_last_death: native GET_TIME_SINCE_LAST_DEATH (handler 0x00BB2050).
//
// No-arg query: calls the death-timer reader, stores the result in the return slot.
export!(cdecl, rw_get_time_since_last_death(ctx: *mut u32) -> u32 {
    unsafe {
        let ans = callee_cdecl!(1, u32,);
        *(*ctx as *mut u32) = ans;
        ans
    }
});
