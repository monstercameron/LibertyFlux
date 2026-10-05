// original: 0x00bd3fc0 GET_IS_HIDEF
/// GET_IS_HIDEF: forward 0 script arguments to the engine implementation and store its low byte (zero-extended) in the script return slot.
lf_rn101_rt::export!(cdecl, rw_fn_00bd3fc0(ctx: u32) -> u32 {
    let r = lf_rn101_rt::callee_cdecl!(1, u32,);
    let slot = unsafe { *(ctx as *const u32) };
    unsafe { *((slot) as *mut u32) = r & 0xff; }
    slot // exit eax is the slot pointer (original ends (an instruction of the original))
});
