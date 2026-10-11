// original: 0x00DCD860 aim_task_destroy_attached_resources
/// Releases the optional task-list reference, then releases the three
/// attached aim resources in order and delegates to the base destructor.
/// Each resource release passes the raw -1000.0f word and then the task
/// address. Null resource fields skip both helper calls and remain zero.
lf_checker_rt::export!(thiscall, rw_00dcd860(task: u32) -> () {
    const TASK_VTABLE_VA: u32 = 0x00ef9974;
    const TASK_LIST_OFFSET: usize = 0x14;
    const RESOURCE_OFFSETS: [usize; 3] = [0x64, 0xac, 0x68];
    const RELEASE_LIST_ID: u32 = 1;
    const RELEASE_RESOURCE_ID: u32 = 2;
    const DETACH_RESOURCE_ID: u32 = 3;
    const BASE_DESTROY_ID: u32 = 4;
    const RELEASE_RATE_BITS: u32 = 0xc47a0000;

    // SAFETY: the contract supplies a writable task object.
    unsafe { (task as *mut u32).write(lf_checker_rt::relocated(TASK_VTABLE_VA)); }
    let task_list_slot = task.wrapping_add(TASK_LIST_OFFSET as u32);
    // SAFETY: the contract supplies readable and writable task storage.
    let task_list = unsafe { (task_list_slot as *const u32).read() };
    if task_list != 0 {
        let _ = lf_checker_rt::callee_thiscall!(RELEASE_LIST_ID, u32, task_list, task_list_slot);
        // SAFETY: the contract supplies a writable task object.
        unsafe { (task_list_slot as *mut u32).write(0); }
    }
    for offset in RESOURCE_OFFSETS {
        let slot = task.wrapping_add(offset as u32);
        // SAFETY: the contract supplies readable and writable task storage.
        let resource = unsafe { (slot as *const u32).read() };
        if resource != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_RESOURCE_ID, u32, resource, RELEASE_RATE_BITS);
            let _ = lf_checker_rt::callee_thiscall!(DETACH_RESOURCE_ID, u32, resource, task);
            // SAFETY: the contract supplies writable storage at this field.
            unsafe { (slot as *mut u32).write(0); }
        }
    }
    let _ = lf_checker_rt::callee_thiscall!(BASE_DESTROY_ID, u32, task);
});
