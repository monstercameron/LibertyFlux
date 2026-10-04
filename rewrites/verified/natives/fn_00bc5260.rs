// original: 0x00BC5260 BURST_CAR_TYRE
// BURST_CAR_TYRE: forwards (car, tyre) to the engine. No return.
export!(cdecl, rw_fn_bc5260(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let _ans: u32 = callee_cdecl!(1, u32, *args, *args.add(1));
    }
});
