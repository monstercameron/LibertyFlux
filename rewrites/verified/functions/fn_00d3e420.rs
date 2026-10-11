// original: 0x00D3E420 task_registry_assign_identifier (proposed)
/// Replaces the task identifier when it differs from the object's signed
/// 16-bit identifier, releasing the old task first when its active bit is
/// set. Identifier -1 clears the stored task pointer; other identifiers are
/// resolved through the current task factory. A missing task pointer returns
/// zero. The helper then asks the task manager whether the task may be
/// assigned; a rejected task is registered as failed and returns zero. An
/// accepted task is registered once, its active bit is set, and the method
/// returns one. The singleton pointers are read from their global slots.
lf_checker_rt::export!(thiscall, rw_d3e420(this: u32, task_id: u32) -> u8 {
    const RELEASE_TASK: u32 = 1;
    const CREATE_TASK: u32 = 2;
    const IS_ASSIGNABLE: u32 = 3;
    const REGISTER_FAILED: u32 = 4;
    const REGISTER_ACTIVE: u32 = 5;
    const TASK_FACTORY_SLOT: u32 = 0x016D_D63C;
    const TASK_MANAGER_SLOT: u32 = 0x0104_96E8;
    const TASK_POINTER_OFFSET: usize = 4;
    const TASK_ID_OFFSET: usize = 8;
    const FLAGS_OFFSET: usize = 10;
    const ACTIVE: u16 = 1;
    const NO_TASK: u16 = u16::MAX;

    let object = this as *mut u8;
    let old_id = unsafe { (object.add(TASK_ID_OFFSET) as *const u16).read_unaligned() };
    if (old_id as i16 as i32 as u32) != task_id {
        let old_flags = unsafe { (object.add(FLAGS_OFFSET) as *const u16).read_unaligned() };
        if old_flags & ACTIVE != 0 {
            let old_task = unsafe { (object.add(TASK_POINTER_OFFSET) as *const u32).read_unaligned() };
            let _ = lf_checker_rt::callee_cdecl!(RELEASE_TASK, u32, old_task);
            unsafe { (object.add(FLAGS_OFFSET) as *mut u16).write_unaligned(old_flags & !ACTIVE); }
        }
        unsafe { (object.add(TASK_ID_OFFSET) as *mut u16).write_unaligned(task_id as u16); }
        if task_id as u16 == NO_TASK {
            unsafe { (object.add(TASK_POINTER_OFFSET) as *mut u32).write_unaligned(u32::MAX); }
        } else {
            let factory = unsafe { lf_checker_rt::global::<u32>(TASK_FACTORY_SLOT).read_unaligned() };
            let signed_id = task_id as u16 as i16 as i32 as u32;
            let new_task = lf_checker_rt::callee_thiscall!(CREATE_TASK, u32, factory, signed_id);
            unsafe { (object.add(TASK_POINTER_OFFSET) as *mut u32).write_unaligned(new_task); }
        }
    }

    let task = unsafe { (object.add(TASK_POINTER_OFFSET) as *const u32).read_unaligned() };
    if task == u32::MAX {
        return 0;
    }
    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER_SLOT).read_unaligned() };
    let accepted = lf_checker_rt::callee_cdecl!(IS_ASSIGNABLE, u8, task, manager);
    if accepted == 0 {
        let _ = lf_checker_rt::callee_cdecl!(REGISTER_FAILED, u8, task, manager, 0);
        return 0;
    }

    let flags = unsafe { (object.add(FLAGS_OFFSET) as *const u16).read_unaligned() };
    if flags & ACTIVE == 0 {
        let _ = lf_checker_rt::callee_cdecl!(REGISTER_ACTIVE, u8, task);
        unsafe { (object.add(FLAGS_OFFSET) as *mut u16).write_unaligned(flags | ACTIVE); }
    }
    1
});
