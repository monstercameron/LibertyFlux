// original: 0x00DCD130 CTaskSimpleReloadGun::vf1
/// Queries the global reload manager and returns zero when it is absent.
/// Otherwise it initializes this task using the identifier and exact float
/// words stored in its fields at offsets 0x60, 0x40, and 0x44. Both direct
/// calls are scripted by the checker.
lf_checker_rt::export!(thiscall, rw_00dcd130(task: u32) -> u32 {
    const GET_MANAGER_ID: u32 = 1;
    const INIT_TASK_ID: u32 = 2;
    const MANAGER_GLOBAL_VA: u32 = 0x0167e2a0;
    const IDENTIFIER_OFFSET: usize = 0x60;
    const INITIAL_RATE_OFFSET: usize = 0x40;
    const FINAL_RATE_OFFSET: usize = 0x44;

    // SAFETY: the checker maps the original image's global data for this read.
    let global_manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read() };
    let manager = lf_checker_rt::callee_thiscall!(GET_MANAGER_ID, u32, global_manager);
    if manager == 0 {
        return 0;
    }
    // SAFETY: the contract supplies the task fields as readable object storage.
    let (identifier, initial_rate_bits, final_rate_bits) = unsafe {
        let bytes = task as *const u8;
        (
            (bytes.add(IDENTIFIER_OFFSET) as *const u32).read(),
            (bytes.add(INITIAL_RATE_OFFSET) as *const u32).read(),
            (bytes.add(FINAL_RATE_OFFSET) as *const u32).read(),
        )
    };
    lf_checker_rt::callee_thiscall!(
        INIT_TASK_ID, u32, manager, identifier, initial_rate_bits, final_rate_bits
    )
});
