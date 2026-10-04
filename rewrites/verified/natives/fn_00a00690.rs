// original: 0x00a00690 CREATE_MONEY_PICKUP
/// CREATE_MONEY_PICKUP: Creates a money pickup with the specified cash value; forward 6 script arguments to the engine implementation.
/// Argument 4 is coerced to 0/1; the pushed word keeps the context
/// pointer's high bytes (the original reuses its incoming stack slot).
/// Float arguments are forwarded as raw bits (bit-exact by construction).
lf_rn109_rt::export!(cdecl, rw_fn_00a00690(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) }; // float bits
    let a1 = unsafe { *((args + 4) as *const u32) }; // float bits
    let a2 = unsafe { *((args + 8) as *const u32) }; // float bits
    let a3 = unsafe { *((args + 12) as *const u32) };
    let a4 = unsafe { *((args + 16) as *const u32) };
    let a5 = unsafe { *((args + 20) as *const u32) };
    let q4 = (ctx & 0xffff_ff00) | u32::from(a4 != 0);
    lf_rn109_rt::callee_cdecl!(1, u32, a0, a1, a2, a3, q4, a5,)
});
