// original: 0x00E6D020 clear_vehicle_task_state_block_00
/// Calls the vehicle-state helper with the designated global object, then registers its cleanup callback. The callback registration result is returned in EAX.
const OBJECT_VA: u32 = 0x017A3890;
const FIRST_ID: u32 = 1;
const ATEXIT_ID: u32 = 2;
const CLEANUP_VA: u32 = 0x00E72DF0;
lf_checker_rt::export!(cdecl, rw_00e6d020() -> u32 {
    let object = lf_checker_rt::relocated(OBJECT_VA);
    let _ = lf_checker_rt::callee_thiscall!(FIRST_ID, u32, object);
    lf_checker_rt::callee_cdecl!(ATEXIT_ID, u32, lf_checker_rt::relocated(CLEANUP_VA))
});
