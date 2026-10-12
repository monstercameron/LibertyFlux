// original: 0x00A82160 TaskList_FindByStage (proposed)

/// Return the first task in a linked list whose encoded stage matches.
///
/// The `thiscall` receiver points to a list head pointer at `+0x00`. Each task
/// stores its stage in bits 1 through 3 of the dword at `+0x08`, and its next
/// pointer at `+0x0C`. The requested stage is an unmasked stack argument. A
/// null head or exhausted list returns null; matching returns the task pointer.
lf_checker_rt::export!(thiscall, rw_00a82160(task_list: u32, desired_stage: u32) -> u32 {
    const FIRST_TASK: u32 = 0x00;
    const TASK_STAGE: u32 = 0x08;
    const NEXT_TASK: u32 = 0x0c;
    const STAGE_SHIFT: u32 = 1;
    const STAGE_MASK: u32 = 0x07;

    let mut task = unsafe { rd32(task_list.wrapping_add(FIRST_TASK)) };
    while task != 0 {
        let encoded_stage = unsafe { rd32(task.wrapping_add(TASK_STAGE)) };
        let stage = (encoded_stage >> STAGE_SHIFT) & STAGE_MASK;
        if stage == desired_stage {
            return task;
        }
        task = unsafe { rd32(task.wrapping_add(NEXT_TASK)) };
    }

    0
});

#[inline(always)]
unsafe fn rd32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}
