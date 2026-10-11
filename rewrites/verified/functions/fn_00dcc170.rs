// original: 0x00DCC170 CTaskComplexMoveAvoidOtherPedWhileWandering::vf1
/// Looks up the manager, then asks it to update the task's nested object using
/// the pointer fields at offsets 0x20 and 0x40 plus the exact float word at
/// offset 0x18. If task flag bit 2 is set, it forwards that bit to the object
/// returned by the update helper. A missing manager returns zero.
lf_checker_rt::export!(thiscall, rw_00dcc170(task: u32) -> u32 {
    const FIND_MANAGER_ID: u32 = 1;
    const UPDATE_OBJECT_ID: u32 = 2;
    const MANAGER_GLOBAL_VA: u32 = 0x0167e2a0;
    const SOURCE_OBJECT_OFFSET: usize = 0x20;
    const FLOAT_WORD_OFFSET: usize = 0x18;
    const UPDATE_RECORD_OFFSET: usize = 0x40;
    const TASK_FLAGS_OFFSET: usize = 0x88;

    // SAFETY: the checker maps the original image's global data for this read.
    let manager_global = unsafe { lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read() };
    let manager = lf_checker_rt::callee_thiscall!(FIND_MANAGER_ID, u32, manager_global);
    if manager == 0 {
        return 0;
    }
    // SAFETY: the contract provides readable task storage.
    let (source_object, update_record, float_word) = unsafe {
        let bytes = task as *const u8;
        (
            (bytes.add(SOURCE_OBJECT_OFFSET) as *const u32).read(),
            task.wrapping_add(UPDATE_RECORD_OFFSET as u32),
            (bytes.add(FLOAT_WORD_OFFSET) as *const u32).read(),
        )
    };
    let updated_object = lf_checker_rt::callee_thiscall!(
        UPDATE_OBJECT_ID, u32, manager, source_object, update_record, float_word
    );
    // SAFETY: both the task and the returned heap object are writable in the contract.
    unsafe {
        if (task as *const u8).add(TASK_FLAGS_OFFSET).read() & 4 != 0 {
            let flags = (updated_object.wrapping_add(TASK_FLAGS_OFFSET as u32) as *const u8).read();
            (updated_object.wrapping_add(TASK_FLAGS_OFFSET as u32) as *mut u8).write(flags | 4);
        }
    }
    updated_object
});
