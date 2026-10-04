// original: 0x00bc5170 ATTACH_CAR_TO_OBJECT
// ATTACH_CAR_TO_OBJECT: forwards (car, object, flags) plus two by-value
// vectors (offset xyz, rotation xyz) to the engine. No return.
export!(cdecl, rw_fn_bc5170(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let _ans: u32 = callee_cdecl!(
            1, u32, *args, *args.add(1), *args.add(2), *args.add(3),
            *args.add(4), *args.add(5), *args.add(6), *args.add(7), *args.add(8)
        );
    }
});
