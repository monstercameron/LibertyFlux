// original: 0x00bc7e80 SET_VEHICLE_IS_CONSIDERED_BY_PLAYER
/// Script native handler `SET_VEHICLE_IS_CONSIDERED_BY_PLAYER`.
///
/// Forwards the vehicle handle and a coerced boolean flag to the vehicle engine; no script return value.
lf_rn94_rt::export!(cdecl, rw_fn_00bc7e80(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let handle = *(args as *const u32);
        let flag = *((args + 4) as *const u32);
        // The original coerces the flag with `setne` into the low byte of its
        // own incoming stack slot, so the pushed word keeps the slot's high
        // bytes (the context pointer). Reproduced exactly from `ctx`.
        let coerced = (ctx & !0xFF) | ((flag != 0) as u32);
        lf_rn94_rt::callee_cdecl!(1, u32, handle, coerced)
    }
});
