// original: 0x00BA1A70 SET_CHAR_SIGNAL_AFTER_KILL
// SET_CHAR_SIGNAL_AFTER_KILL: forwards (char, coerced signal flag).
export!(cdecl, rw_fn_ba1a70(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let flag = ((*args.add(1) != 0) as u32);
        let _ans: u32 = callee_cdecl!(1, u32, *args, flag);
    }
});
