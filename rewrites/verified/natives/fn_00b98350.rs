// original: 0x00b98350 GET_MOTION_SENSOR_VALUES
/// GET_MOTION_SENSOR_VALUES: Reads the motion sensor values; forward 5 script arguments to the engine implementation and store its low byte (zero-extended) in the script return slot.
lf_rn109_rt::export!(cdecl, rw_fn_00b98350(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let a1 = unsafe { *((args + 4) as *const u32) };
    let a2 = unsafe { *((args + 8) as *const u32) };
    let a3 = unsafe { *((args + 12) as *const u32) };
    let a4 = unsafe { *((args + 16) as *const u32) };
    let r = lf_rn109_rt::callee_cdecl!(1, u32, a0, a1, a2, a3, a4,);
    let slot = unsafe { *(ctx as *const u32) };
    unsafe { *((slot) as *mut u32) = r & 0xff; }
    slot // exit eax is the slot pointer (original ends (an instruction of the original))
});
