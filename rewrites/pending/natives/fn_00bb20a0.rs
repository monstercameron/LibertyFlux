// original: 0x00bb20a0 GET_TIME_SINCE_PLAYER_HIT_BUILDING
// GET_TIME_SINCE_PLAYER_HIT_BUILDING: engine(arg0), store eax in return slot.
export!(cdecl, rw_00bb20a0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let r = callee_cdecl!(1, u32, *a);
        *ret_of(ctx) = r;
        r
    }
});
