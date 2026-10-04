// original: 0x00bc7580 SET_CAR_FORWARD_SPEED
// SET_CAR_FORWARD_SPEED: forwards (car, speed-bits). The speed word moves
// through a vector register in the original, which is just a 32-bit copy.
export!(cdecl, rw_fn_bc7580(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let _ans: u32 = callee_cdecl!(1, u32, *args, *args.add(1));
    }
});
