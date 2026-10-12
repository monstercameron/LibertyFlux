// original: 0x00cfbfa0 CTaskComplexBeArrestedAndDrivenAway::CTaskComplexBeArrestedAndDrivenAway
/// Call the complex-task base constructor, install this task's vtable, and store
/// the related task at offset 0x14. Clear offsets 0x18 and 0x1C, then register a
/// non-null related task with the address of its field. Return this object.
lf_checker_rt::export!(thiscall, rw_00cfbfa0(this: *mut u8, related_task: u32) -> u32 {
    unsafe {
        const TASK_VTABLE: u32 = 0x00EE06EC;
        const RELATED_TASK: usize = 0x14;
        lf_checker_rt::callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = lf_checker_rt::relocated(TASK_VTABLE);
        let related_slot = this.add(RELATED_TASK) as *mut u32;
        *related_slot = related_task;
        *(this.add(0x18) as *mut u32) = 0;
        *(this.add(0x1C) as *mut u32) = 0;
        if related_task != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, related_task, related_slot as u32);
        }
        this as u32
    }
});
