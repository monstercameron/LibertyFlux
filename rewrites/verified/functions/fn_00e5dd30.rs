// original: 0x00e5dd30 M_Y_GIRI_LO_01_FULL_01
/// Resolve the fixed model name to its index handle and publish it.
///
/// Calls the model registry with the fixed name slot, stores the
/// returned handle in the dedicated global, and returns the handle.
/// Takes no inputs.
lf_checker_rt::export!(cdecl, rw_00e5dd30() -> u32 {
    let handle: u32 = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00F8AA38), 0);
    unsafe {
        *lf_checker_rt::global::<u32>(0x019D2FF0) = handle;
    }
    handle
});
