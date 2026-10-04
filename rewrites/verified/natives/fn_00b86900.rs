// original: 0x00b86900 COUNT_SCRIPT_CAMS
/// COUNT_SCRIPT_CAMS: forward 0 script arguments to the engine implementation and store its full result in the script return slot.
lf_rn101_rt::export!(cdecl, rw_fn_00b86900(ctx: u32) -> u32 {
    let r = lf_rn101_rt::callee_cdecl!(1, u32,);
    let slot = unsafe { *(ctx as *const u32) };
    unsafe { *((slot) as *mut u32) = r; }
    r // exit eax keeps the callee answer (slot held in ecx)
});
