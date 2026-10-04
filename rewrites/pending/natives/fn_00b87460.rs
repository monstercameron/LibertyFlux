// original: 0x00B87460 SET_CAM_INTERP_STYLE_DETAILED
// SET_CAM_INTERP_STYLE_DETAILED: forwards 5 script words. No return.
export!(cdecl, rw_fn_b87460(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let _ans: u32 = callee_cdecl!(
            1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4)
        );
    }
});
