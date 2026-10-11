// original: 0x00E6E5A0 mainloop_timing_initialize_tail

/// Stores the relocated initializer pointer in the shared slot, then forwards
/// that slot's address in ECX to the target initializer and returns its result.
pub extern "cdecl" fn rw_00e6e5a0() -> u32 {
    const INITIALIZER_SLOT: u32 = 0x0198_1fa8;
    const INITIALIZER_TARGET: u32 = 0x00fe_4024;

    unsafe {
        lf_checker_rt::global::<u32>(INITIALIZER_SLOT)
            .write(lf_checker_rt::relocated(INITIALIZER_TARGET));
    }
    let initialize: extern "thiscall" fn(u32) -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(1) as usize)
    };
    initialize(lf_checker_rt::relocated(INITIALIZER_SLOT))
}
