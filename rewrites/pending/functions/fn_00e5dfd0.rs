// original: 0x00e5dfd0 F_Y_MPWHITE_01
/// Resolve the fixed model name to its index handle and publish it.
///
/// Calls the model registry with the fixed name slot, stores the
/// returned handle in the dedicated global, and returns the handle.
/// Takes no inputs.
lf_checker_rt::export!(cdecl, rw_00e5dfd0() -> u32 {
    let handle: u32 = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00F8A838), 0);
    unsafe {
        *lf_checker_rt::global::<u32>(0x019D2E04) = handle;
    }
    handle
});
