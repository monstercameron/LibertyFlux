// original: 0x00E6D640 streaming_vector_register_storage_cleanup
/// Registers the fixed cleanup callback with the intercepted atexit wrapper and returns its result in EAX.
const ATEXIT_ID: u32 = 1;
const CLEANUP_VA: u32 = 0x00E72E40;
lf_checker_rt::export!(cdecl, rw_00e6d640() -> u32 {
    lf_checker_rt::callee_cdecl!(ATEXIT_ID, u32, lf_checker_rt::relocated(CLEANUP_VA))
});
