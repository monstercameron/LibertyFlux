// original: 0x00DCD0C0 reload_task_destroy_resource
/// Releases the task's current resource when present, clears its handle,
/// then forwards destruction to the base task. The release helper receives
/// the resource and the exact -1000.0f bit pattern; the second helper gets
/// the task pointer. The stored task vtable is relocated for the worker.
lf_checker_rt::export!(thiscall, rw_00dcd0c0(task: u32) -> () {
    const TASK_VTABLE_VA: u32 = 0x00ef97c4;
    const RESOURCE_OFFSET: usize = 0x48;
    const RELEASE_ID: u32 = 1;
    const DETACH_ID: u32 = 2;
    const BASE_DESTROY_ID: u32 = 3;
    const RELEASE_RATE_BITS: u32 = 0xc47a0000;

    // SAFETY: the contract supplies a writable task object.
    unsafe { (task as *mut u32).write(lf_checker_rt::relocated(TASK_VTABLE_VA)); }
    // SAFETY: the contract supplies readable and writable task storage.
    let resource = unsafe { ((task as *mut u8).add(RESOURCE_OFFSET) as *mut u32).read() };
    if resource != 0 {
        let _ = lf_checker_rt::callee_thiscall!(RELEASE_ID, u32, resource, RELEASE_RATE_BITS);
        let _ = lf_checker_rt::callee_thiscall!(DETACH_ID, u32, resource, task);
        // SAFETY: the contract supplies a writable task object.
        unsafe { ((task as *mut u8).add(RESOURCE_OFFSET) as *mut u32).write(0); }
    }
    let _ = lf_checker_rt::callee_thiscall!(BASE_DESTROY_ID, u32, task);
});
