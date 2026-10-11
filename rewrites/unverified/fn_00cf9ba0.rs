// original: 0x00cf9ba0 climb_task_has_unit_height_subtask_88_8a
/// Returns true when the task at offset 0x08 reports the expected query kind, has a
/// child task whose type is one of the two accepted codes, and stores exactly 1.0f
/// at child offset 0x58. The query uses the object vtable slot at byte offset 0x0C.
/// The result is a C++ bool in AL; upper EAX bytes retain unrelated call residue.
lf_checker_rt::export!(thiscall, rw_00cf9ba0(this: *mut u8) -> u32 {
    unsafe {
        const TASK_SUBTASK: usize = 0x08;
        const QUERY_SLOT: usize = 0x0C;
        const CHILD_TASK: usize = 0x60;
        const CHILD_TYPE: usize = 0x0C;
        const CHILD_HEIGHT: usize = 0x58;
        const QUERY_KIND: u32 = 0x120;
        const ONE_FILE_VA: u32 = 0x00FE88E8;

        let candidate = *(this.add(TASK_SUBTASK) as *const u32);
        if candidate == 0 {
            return 0;
        }
        let vtable = *(candidate as *const *const u32);
        let query_target = *vtable.add((QUERY_SLOT / 4) as usize);
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(query_target);
        if query(candidate) != QUERY_KIND {
            return 0;
        }
        let child = *((candidate as *const u8).add(CHILD_TASK) as *const u32);
        if child == 0 {
            return 0;
        }
        let child_type = *((child as *const u8).add(CHILD_TYPE) as *const u32);
        if child_type != 0x88 && child_type != 0x8A {
            return 0;
        }
        let height = *((child as *const u8).add(CHILD_HEIGHT) as *const f32);
        let one = *lf_checker_rt::global::<f32>(ONE_FILE_VA);
        if height == one {
            1
        } else {
            0
        }
    }
});
