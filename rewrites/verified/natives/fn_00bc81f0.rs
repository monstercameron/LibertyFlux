// original: 0x00BC81F0 SWITCH_CAR_SIREN
// SWITCH_CAR_SIREN: forwards (car, coerced siren flag).
export!(cdecl, rw_fn_bc81f0(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let flag = ((*args.add(1) != 0) as u32);
        let _ans: u32 = callee_cdecl!(1, u32, *args, flag);
    }
});
