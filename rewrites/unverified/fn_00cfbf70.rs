// original: 0x00cfbf70 CTaskComplexBackOff::CTaskComplexBackOff
/// Call the complex-task base constructor, install this task's vtable, and store
/// the related task at offset 0x14. Clear offset 0x18 and register a non-null
/// related task with the address of its field. Return this object.
lf_checker_rt::export!(thiscall, rw_00cfbf70(this: *mut u8, related_task: u32) -> u32 {
    unsafe {
        const TASK_VTABLE: u32 = 0x00EE0694;
        const RELATED_TASK: usize = 0x14;
        lf_checker_rt::callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = lf_checker_rt::relocated(TASK_VTABLE);
        *(this.add(RELATED_TASK) as *mut u32) = related_task;
        *this.add(0x18) = 0;
        if related_task != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, related_task, this.add(RELATED_TASK) as u32);
        }
        this as u32
    }
});
