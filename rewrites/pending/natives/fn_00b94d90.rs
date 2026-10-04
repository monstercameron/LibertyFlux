// original: 0x00b94d90 SET_EXTRA_HOSPITAL_RESTART_POINT
// SET_EXTRA_HOSPITAL_RESTART_POINT: forwards 5 float words by value. Each
// moves through a vector register in the original, i.e. a 32-bit copy.
export!(cdecl, rw_fn_b94d90(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let _ans: u32 = callee_cdecl!(
            1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4)
        );
    }
});
