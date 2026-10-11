// original: 0x00B711B0 fn_00b711b0
/// Checks a task's linked target and, for the confirmed special target kind,
/// sets the target's enabled flag and two float-related fields before asking
/// the task system to transition. `task` is a this pointer; `task_kind` and
/// `argument` are the two stack words. Queries use scripted virtual calls;
/// the final helper is thiscall with three stack words. Only the low return
/// byte is a success flag.
lf_checker_rt::export!(thiscall, rw_00b711b0(task: u32, task_kind: u32, argument: u32) -> u8 {
    unsafe {
        const TARGET_WEIGHT: u32 = 0x00EE_1ED8;
        const TASK_OBJECT: u32 = 0x08;
        const ARGUMENT_OWNER: u32 = 0x224;
        const TARGET_FLAGS: u32 = 0xD8;
        const TARGET_WEIGHT_FIELD: u32 = 0x60;
        const TARGET_LIMIT_FIELD: u32 = 0x64;
        const ENABLED_FLAG: u32 = 0x0010_0000;
        let task_object = core::ptr::read((task + TASK_OBJECT) as *const u32);
        if task_object == 0 { return 0; }
        if lf_checker_rt::callee_thiscall!(1, u32, task_object) != 0x11D { return 0; }
        let argument_owner = core::ptr::read((argument + ARGUMENT_OWNER) as *const u32);
        let target = lf_checker_rt::callee_thiscall!(2, u32, argument_owner);
        if target == 0 { return 0; }
        if lf_checker_rt::callee_thiscall!(3, u32, target) != task_kind { return 0; }
        let target_kind = lf_checker_rt::callee_thiscall!(3, u32, target);
        let should_enable = if target_kind == 0x384 {
            lf_checker_rt::callee_thiscall!(3, u32, target) == 0x3AE
        } else {
            if target_kind != 0x3AE { return 0; }
            if lf_checker_rt::callee_thiscall!(3, u32, target) != 0x3AE { return 0; }
            lf_checker_rt::callee_thiscall!(3, u32, target) == 0x3AE
        };
        if should_enable {
            let flags = core::ptr::read((target + TARGET_FLAGS) as *const u32);
            core::ptr::write((target + TARGET_FLAGS) as *mut u32, flags | ENABLED_FLAG);
            let weight = core::ptr::read(lf_checker_rt::global::<f32>(TARGET_WEIGHT));
            core::ptr::write((target + TARGET_WEIGHT_FIELD) as *mut f32, weight);
            core::ptr::write((target + TARGET_LIMIT_FIELD) as *mut u32, 0xC47A_0000);
        }
        let _ = lf_checker_rt::callee_thiscall!(4, u32, task_object, argument, task + 0x60, 0);
        1
    }
});
