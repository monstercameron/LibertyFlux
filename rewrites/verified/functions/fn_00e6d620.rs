// original: 0x00E6D620 random_point_workspace_allocate_and_register_cleanup
/// Requests an 80-byte workspace from the intercepted allocator, stores the returned pointer globally, registers the cleanup callback, and returns the registration result in EAX.
const ALLOC_ID: u32 = 1;
const ATEXIT_ID: u32 = 2;
const ALLOCATION_BYTES: u32 = 0x50;
const CLEANUP_VA: u32 = 0x00E72E20;
const WORKSPACE_VA: u32 = 0x010570C4;
lf_checker_rt::export!(cdecl, rw_00e6d620() -> u32 {
    let workspace = lf_checker_rt::callee_cdecl!(ALLOC_ID, u32, ALLOCATION_BYTES);
    unsafe { lf_checker_rt::global::<u32>(WORKSPACE_VA).write(workspace) };
    lf_checker_rt::callee_cdecl!(ATEXIT_ID, u32, lf_checker_rt::relocated(CLEANUP_VA))
});
