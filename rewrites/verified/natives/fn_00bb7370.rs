// original: 0x00bb7370 GET_VEHICLE_MODEL_FROM_INDEX
/// GET_VEHICLE_MODEL_FROM_INDEX: forward 2 script arguments to the engine implementation and store its low byte (zero-extended) in the script return slot.
lf_rn101_rt::export!(cdecl, rw_fn_00bb7370(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let a1 = unsafe { *((args + 4) as *const u32) };
    let r = lf_rn101_rt::callee_cdecl!(1, u32, a0, a1,);
    let slot = unsafe { *(ctx as *const u32) };
    unsafe { *((slot) as *mut u32) = r & 0xff; }
    slot // exit eax is the slot pointer (original ends (an instruction of the original))
});
