// original: 0x00D28040 task_manager_add_priority_tasks (proposed)

/// Inspect up to sixteen candidate task pointers. For each non-null candidate,
/// call the manager's virtual eligibility hook. Eligible candidates are
/// checked by the priority helper; a non-null returned task triggers its
/// completion helper. If no priority task is returned, the manager's add-task
/// path runs, followed by the fallback predicate and, when accepted, a second
/// add-task call. Return false only when that fallback predicate rejects;
/// otherwise return true after continuing or exhausting the list. The
/// thiscall method has one stack pointer.
lf_checker_rt::export!(thiscall, rw_00D28040(this: u32, candidates: u32) -> u8 {
    const PENDING_TASK: u32 = 0x238;
    const VTABLE_POINTER: u32 = 0;
    const ELIGIBILITY_SLOT: u32 = 0x1c;
    const CANDIDATE_ARRAY: u32 = 0x18;
    const CANDIDATE_COUNT: u32 = 16;
    const ELIGIBILITY_CALLEE: u32 = 1;
    const PRIORITY_CALLEE: u32 = 2;
    const COMPLETE_CALLEE: u32 = 3;
    const ADD_CALLEE: u32 = 4;
    const FALLBACK_CALLEE: u32 = 5;

    unsafe {
        if crate::read_u32(this.wrapping_add(PENDING_TASK)) != 0 {
            return 1;
        }

        let vtable = crate::read_u32(this.wrapping_add(VTABLE_POINTER));
        let eligible_slot = crate::read_u32(vtable.wrapping_add(ELIGIBILITY_SLOT));
        let eligible: extern "thiscall" fn(u32, u32) -> u8 =
            core::mem::transmute(eligible_slot as usize);

        for index in 0..CANDIDATE_COUNT {
            let task = crate::read_u32(
                candidates
                    .wrapping_add(CANDIDATE_ARRAY)
                    .wrapping_add(index.wrapping_mul(4)),
            );
            if task == 0 || eligible(this, task) == 0 {
                continue;
            }

            let priority_task =
                lf_checker_rt::callee_thiscall!(PRIORITY_CALLEE, u32, this, task);
            if priority_task != 0 {
                let _ = lf_checker_rt::callee_thiscall!(COMPLETE_CALLEE, u32, priority_task);
                continue;
            }

            if lf_checker_rt::callee_thiscall!(ADD_CALLEE, u8, this, task) != 0 {
                continue;
            }
            if lf_checker_rt::callee_thiscall!(FALLBACK_CALLEE, u8, this) == 0 {
                return 0;
            }
            let _ = lf_checker_rt::callee_thiscall!(ADD_CALLEE, u8, this, task);
        }
        1
    }
});
