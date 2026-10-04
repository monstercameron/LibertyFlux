// original: 0x00bd78c0 FIND_NETWORK_KILLER_OF_PLAYER
/// FIND_NETWORK_KILLER_OF_PLAYER: forward 1 script argument to the engine implementation and store its full result in the script return slot.
lf_rn101_rt::export!(cdecl, rw_fn_00bd78c0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let r = lf_rn101_rt::callee_cdecl!(1, u32, a0,);
    let slot = unsafe { *(ctx as *const u32) };
    unsafe { *((slot) as *mut u32) = r; }
    r // exit eax keeps the callee answer (slot held in ecx)
});
