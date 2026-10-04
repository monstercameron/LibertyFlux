// original: 0x00ba0ff0 SET_CHAR_ANIM_PLAYING_FLAG
// SET_CHAR_ANIM_PLAYING_FLAG: forwards (char, a, b) plus the coerced bool
// flag word, built in the dead argument slot like the other bool handlers.
export!(cdecl, rw_fn_ba0ff0(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let flag = (ctx as u32 & 0xFFFFFF00) | ((*args.add(3) != 0) as u32);
        let _ans: u32 = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), flag);
    }
});
