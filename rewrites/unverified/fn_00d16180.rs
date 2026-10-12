// original: 0x00D16180 CTaskComplexCombatAdvanceSubtask::select_sub_task_id

/// Select the next combat-advance task id from the incoming id, an object flag,
/// and two helper answers. ECX points to the task; stack arguments are a task
/// pointer and a signedness-neutral id word. The low flag byte at +0x40 can
/// immediately choose the blocked id for three specific inputs. Otherwise the
/// function asks whether cover is available, conditionally asks whether the
/// embedded task is aim-facing-clear, and returns either the blocked id or the
/// original id. Both helper calls use cdecl and their byte answers are observed
/// in AL; thiscall consumes both input stack words.
lf_checker_rt::export!(thiscall, rw_D16180(this: u32, task_arg: u32, task_id: u32) -> u32 {
    const FLAGS_BYTE: u32 = 0x40;
    const SUBTASK_POINTER: u32 = 0x18;
    const COVER_BLOCKED: u32 = 0x785;
    const ID_BLOCKED_A: u32 = 0x781;
    const ID_BLOCKED_B: u32 = 0x78B;
    const ID_BLOCKED_C: u32 = 0x780;
    const RETURN_BLOCKED: u32 = 0x780;

    let flags = unsafe { core::ptr::read((this + FLAGS_BYTE) as *const u8) };
    if flags & 8 != 0
        && (task_id == ID_BLOCKED_A || task_id == ID_BLOCKED_B || task_id == ID_BLOCKED_C)
    {
        return COVER_BLOCKED;
    }

    let cover_available = unsafe { lf_checker_rt::callee_cdecl!(1, u8, task_arg) } != 0;
    if !cover_available && (task_id == ID_BLOCKED_A || task_id == ID_BLOCKED_B) {
        return RETURN_BLOCKED;
    }

    let subtask = unsafe {
        core::ptr::read_unaligned((this + SUBTASK_POINTER) as *const u32)
    };
    if subtask != 0 {
        let aim_facing_clear = unsafe { lf_checker_rt::callee_cdecl!(2, u8, task_arg, subtask) } != 0;
        if aim_facing_clear && task_id == ID_BLOCKED_B {
            return RETURN_BLOCKED;
        }
    }

    task_id
});
