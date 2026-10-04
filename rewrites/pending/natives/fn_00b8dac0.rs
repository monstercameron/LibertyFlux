// original: 0x00b8dac0 SET_TEXT_WRAP
// SET_TEXT_WRAP: forwards 2 float words by value (32-bit copies).
export!(cdecl, rw_fn_b8dac0(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let _ans: u32 = callee_cdecl!(1, u32, *args, *args.add(1));
    }
});
