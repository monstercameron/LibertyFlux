// original: 0x00bc7400 SET_CAR_CAN_BE_VISIBLY_DAMAGED
/// SET_CAR_CAN_BE_VISIBLY_DAMAGED: Sets whether the vehicle can be visibly damaged; forward 2 script arguments to the engine implementation.
/// Argument 1 is coerced to 0/1; the pushed word keeps the context
/// pointer's high bytes (the original reuses its incoming stack slot).
lf_rn109_rt::export!(cdecl, rw_fn_00bc7400(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let a1 = unsafe { *((args + 4) as *const u32) };
    let q1 = (ctx & 0xffff_ff00) | u32::from(a1 != 0);
    lf_rn109_rt::callee_cdecl!(1, u32, a0, q1,)
});
