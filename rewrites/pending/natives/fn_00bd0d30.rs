// original: 0x00BD0D30 ADD_AMMO_TO_CHAR
// ADD_AMMO_TO_CHAR: forwards (char, weapon, amount) to the engine. No return.
export!(cdecl, rw_fn_bd0d30(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let _ans: u32 = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
    }
});
