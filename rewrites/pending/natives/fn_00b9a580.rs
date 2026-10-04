// original: 0x00b9a580 GET_CLOSEST_CAR_NODE_WITH_HEADING
/// Script native handler `GET_CLOSEST_CAR_NODE_WITH_HEADING`.
///
/// Forwards a position plus four node-search parameters to the path engine and stores the boolean answer (low byte) in the script return slot.
lf_rn94_rt::export!(cdecl, rw_fn_00b9a580(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let a0 = *((args + 0) as *const u32);
        let a1 = *((args + 4) as *const u32);
        let a2 = *((args + 8) as *const u32);
        let a3 = *((args + 12) as *const u32);
        let a4 = *((args + 16) as *const u32);
        let a5 = *((args + 20) as *const u32);
        let a6 = *((args + 24) as *const u32);
        let answer = lf_rn94_rt::callee_cdecl!(1, u32, a0, a1, a2, a3, a4, a5, a6);
        let slot = *(ctx as *const u32);
        *((slot) as *mut u32) = answer & 0xFF;
        slot
    }
});
