// original: 0x00e5e030 F_Y_MPBLACK_02
/// Resolve the fixed model name to its index handle and publish it.
///
/// Calls the model registry with the fixed name slot, stores the
/// returned handle in the dedicated global, and returns the handle.
/// Takes no inputs.
lf_checker_rt::export!(cdecl, rw_00e5e030() -> u32 {
    let handle: u32 = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00F8A868), 0);
    unsafe {
        *lf_checker_rt::global::<u32>(0x019D2FE4) = handle;
    }
    handle
});
