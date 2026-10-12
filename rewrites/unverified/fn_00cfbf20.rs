// original: 0x00cfbf20 CTaskComplexArrestedAIPedAndDriveAway::CTaskComplexArrestedAIPedAndDriveAway
/// Call the complex-task base constructor, install this task's vtable, and store
/// the related task at offset 0x14. Clear offsets 0x18 through 0x20, set offset
/// 0x24 to 0x40000000, and register a non-null related task with its field address.
/// Return this object.
lf_checker_rt::export!(thiscall, rw_00cfbf20(this: *mut u8, related_task: u32) -> u32 {
    unsafe {
        const TASK_VTABLE: u32 = 0x00EE0744;
        const RELATED_TASK: usize = 0x14;
        lf_checker_rt::callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = lf_checker_rt::relocated(TASK_VTABLE);
        let related_slot = this.add(RELATED_TASK) as *mut u32;
        *related_slot = related_task;
        let word = |offset: usize| this.add(offset) as *mut u32;
        *word(0x18) = 0;
        *word(0x1C) = 0;
        *word(0x20) = 0;
        *word(0x24) = 0x4000_0000;
        if related_task != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, related_task, related_slot as u32);
        }
        this as u32
    }
});
