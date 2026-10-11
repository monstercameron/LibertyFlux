// original: 0x00D41D10 CTaskSimpleSidewaysDive::vf17
/// Checks the task's disable bit and optional actor state. When enabled it
/// queries the actor's vtable slot at 0x128, asks the actor helper, then tests
/// bit 0x4000 in the task's linked object at offset 0x74. The early path also
/// derives category 0xB7 or 0xB8 from the nonzero word at task offset 0x14,
/// makes the availability query, and optionally submits the actor with a
/// stack float value of 4.0. A missing linked object skips the actor checks and
/// still returns one; failed checks on a present linked object return zero.
lf_checker_rt::export!(thiscall, rw_d41d10(task: u32, actor: u32) -> u8 {
    const QUERY_AVAILABLE: u32 = 2;
    const SUBMIT_ACTOR: u32 = 3;
    const QUERY_ACTOR: u32 = 1;
    const CHECK_ACTOR: u32 = 4;
    const APPLY_DIVE: u32 = 5;
    const TASK_DISABLE_OFFSET: usize = 0x0C;
    const TASK_STATUS_OFFSET: usize = 0x14;
    const TASK_ACTOR_OFFSET: usize = 0x10;
    const ACTOR_FLAGS_OFFSET: usize = 0x74;
    const DIVE_BIT: u32 = 0x4000;
    const CATEGORY_BASE: u32 = 0xB7;
    const ACTOR_EVENT: u32 = 0x8A;
    const SUBMIT_DURATION: u32 = 0x40800000;
    const DIVE_DURATION: u32 = 0xC1000000;
    let object = task as *const u8;
    let disabled = unsafe { (object.add(TASK_DISABLE_OFFSET) as *const u32).read_unaligned() } & 4 != 0;
    if !disabled {
        let status = unsafe { (object.add(TASK_STATUS_OFFSET) as *const u32).read_unaligned() };
        let category = CATEGORY_BASE + u32::from(status != 0);
        let available = lf_checker_rt::callee_cdecl!(QUERY_AVAILABLE, u32, ACTOR_EVENT, category);
        if available != 0 {
            let _ = lf_checker_rt::callee_thiscall!(
                SUBMIT_ACTOR, u32, task, actor, ACTOR_EVENT, category, SUBMIT_DURATION, 1
            );
        }
    }

    let linked_actor = unsafe { (object.add(TASK_ACTOR_OFFSET) as *const u32).read_unaligned() };
    if linked_actor == 0 { return 1; }
    if lf_checker_rt::callee_thiscall!(QUERY_ACTOR, u8, actor) == 0 { return 0; }
    if lf_checker_rt::callee_thiscall!(CHECK_ACTOR, u8, actor) == 0 { return 0; }
    let linked_flags = unsafe {
        ((linked_actor + ACTOR_FLAGS_OFFSET as u32) as *const u32).read_unaligned()
    };
    if linked_flags & DIVE_BIT == 0 { return 0; }
    let _ = lf_checker_rt::callee_thiscall!(APPLY_DIVE, u8, task, DIVE_DURATION);
    1
});
