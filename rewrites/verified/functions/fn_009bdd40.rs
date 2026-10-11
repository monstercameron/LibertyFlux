// original: 0x009BDD40 ped_task_event_registration
/// Registers the task event descriptor by calling the scripted cdecl helper
/// with the relocated descriptor address, a zero option, and size 0xc80.
/// The original helper call is the function's only observable action; it has
/// no semantic return value.
lf_checker_rt::export!(cdecl, rw_009bdd40() -> () {
    const DESCRIPTOR_SIZE: u32 = 0xC80;
    const REGISTRATION_OPTIONS: u32 = 0;
    const DESCRIPTOR_FILE_VA: u32 = 0x0128_E960;

    let descriptor = lf_checker_rt::relocated(DESCRIPTOR_FILE_VA);
    let _ = lf_checker_rt::callee_cdecl!(
        1,
        u32,
        descriptor,
        REGISTRATION_OPTIONS,
        DESCRIPTOR_SIZE,
    );
});
