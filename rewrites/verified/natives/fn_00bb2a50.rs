// original: 0x00BB2A50 SET_PLAYER_CONTROL
// SET_PLAYER_CONTROL: forwards (player, coerced control flag).
export!(cdecl, rw_fn_bb2a50(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let flag = ((*args.add(1) != 0) as u32);
        let _ans: u32 = callee_cdecl!(1, u32, *args, flag);
    }
});
