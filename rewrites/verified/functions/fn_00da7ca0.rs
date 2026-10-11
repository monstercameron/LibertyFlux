// original: 0x00DA7CA0 CTaskComplexEscapeBlast::vf18

/// Return no task unless the actor's virtual type is one of the two
/// blast-escape types and the task's one-byte state is clear. For those
/// types, query the manager singleton and request a task with priority
/// zero, duration 0x0098967F, and mode -1. The actor is stored at `this+8`;
/// its type query is virtual slot 3. The method is thiscall and pops its
/// one explicit stack argument on return.
lf_checker_rt::export!(thiscall, rw_00DA7CA0(this: u32, _actor_argument: u32) -> u32 {
    unsafe {
        const ACTOR_LINK: u32 = 0x08;
        const VTABLE_TYPE_SLOT: u32 = 0x0C;
        const TASK_STATE: u32 = 0x38;
        const BLAST_ESCAPE_FIRST: u32 = 0x38E;
        const BLAST_ESCAPE_LAST: u32 = 0x38F;
        const MANAGER_GLOBAL_VA: u32 = 0x0167E2A0;
        const MANAGER_LOOKUP: u32 = 2;
        const TASK_FACTORY: u32 = 3;
        const TASK_DURATION: u32 = 0x0098967F;

        let actor = (this.wrapping_add(ACTOR_LINK) as *const u32).read_unaligned();
        let vtable = (actor as *const u32).read_unaligned();
        let type_query = (vtable.wrapping_add(VTABLE_TYPE_SLOT) as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(type_query as usize);
        let actor_type = query(actor);
        let task_state = (this.wrapping_add(TASK_STATE) as *const u8).read();

        if (BLAST_ESCAPE_FIRST..=BLAST_ESCAPE_LAST).contains(&actor_type) && task_state == 0 {
            let singleton_context = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
            let manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, singleton_context);
            if manager != 0 {
                return lf_checker_rt::callee_thiscall!(
                    TASK_FACTORY, u32, manager, 0u32, TASK_DURATION, u32::MAX
                );
            }
        }
        0
    }
});
