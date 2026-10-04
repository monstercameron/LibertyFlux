// original: 0x00bd8720 NETWORK_IS_BEING_KICKED
/// Script native handler `NETWORK_IS_BEING_KICKED`.
///
/// Calls the network engine with no arguments and stores the boolean answer (low byte) in the script return slot.
lf_rn94_rt::export!(cdecl, rw_fn_00bd8720(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32);
        let answer = lf_rn94_rt::callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32);
        *((slot) as *mut u32) = answer & 0xFF;
        slot
    }
});
