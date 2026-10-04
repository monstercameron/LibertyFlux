// original: 0x00e616e0 timing_worker_init_and_register
/// Initialise the timing worker, then register its callback.
///
/// Returns the registrar's answer.
export!(cdecl, rw_00e616e0() -> u32 {
    unsafe {
        callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(0x00E70260))
    }
});
