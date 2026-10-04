// original: 0x00a00670 COUNT_PICKUPS_OF_TYPE
/// COUNT_PICKUPS_OF_TYPE: Counts the pickups of a given type; forward 1 script argument to the engine implementation and store its full result in the script return slot.
lf_rn109_rt::export!(cdecl, rw_fn_00a00670(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let r = lf_rn109_rt::callee_cdecl!(1, u32, a0,);
    let slot = unsafe { *(ctx as *const u32) };
    unsafe { *((slot) as *mut u32) = r; }
    r // exit eax keeps the callee answer (slot held in ecx)
});
