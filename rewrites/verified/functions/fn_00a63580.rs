// original: 0x00A63580 find_look_or_pathing_task

/// Search the task chain at `this + 0x2e0` first for type `0x2de` and
/// forward the chain-head address, type and count to the helper. If that type
/// is absent, search again for type `0x2c5` and return the word at the ped
/// entity's `+0xb30`; return zero if neither type is present. The original
/// reads each node's flags word twice without changing it, so the extracted
/// values are equal and both type searches visit every linked node.
lf_checker_rt::export!(thiscall, rw_00a63580(this: u32) -> u32 {
    const TASK_HEAD: u32 = 0x2e0;
    const TASK_TYPE: u32 = 0x04;
    const TASK_NEXT: u32 = 0x0c;
    const QUERY_TYPE: u32 = 0x2de;
    const QUERY_LIMIT: u32 = 5;
    const FALLBACK_TYPE: u32 = 0x2c5;
    const PED_ENTITY: u32 = 0x40;
    const ENTITY_RESULT: u32 = 0xb30;
    let head_slot = this.wrapping_add(TASK_HEAD);
    let head = unsafe { (head_slot as *const u32).read_unaligned() };
    let mut task = head;
    while task != 0 {
        let task_type = unsafe { (task.wrapping_add(TASK_TYPE) as *const u32).read_unaligned() };
        if task_type == QUERY_TYPE {
            return lf_checker_rt::callee_thiscall!(1, u32, head_slot, QUERY_TYPE, QUERY_LIMIT);
        }
        task = unsafe { (task.wrapping_add(TASK_NEXT) as *const u32).read_unaligned() };
    }
    task = head;
    while task != 0 {
        let task_type = unsafe { (task.wrapping_add(TASK_TYPE) as *const u32).read_unaligned() };
        if task_type == FALLBACK_TYPE {
            let entity = unsafe { (this.wrapping_add(PED_ENTITY) as *const u32).read_unaligned() };
            return unsafe { (entity.wrapping_add(ENTITY_RESULT) as *const u32).read_unaligned() };
        }
        task = unsafe { (task.wrapping_add(TASK_NEXT) as *const u32).read_unaligned() };
    }
    0
});
