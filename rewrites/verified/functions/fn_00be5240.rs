// original: 0x00BE5240 task_equal_child_type_tail_lookup

/// Continue when the task flags have bit 2 set and bit 1 clear, then query a
/// task through the context manager and compare its type with the task's child
/// type. Equal types tail-call the manager helper with selector one; all other
/// paths return zero. The tail-call path overwrites the incoming argument slot
/// with that selector, so the contract observes it as the helper's call word.
lf_checker_rt::export!(thiscall, rw_00be5240(this: u32, context: u32) -> u32 {
    unsafe {
        const TASK_FLAGS: u32 = 0x0c;
        const CHILD_OBJECT: u32 = 0x14;
        const CONTEXT_MANAGER: u32 = 0x224;
        const MANAGER_TASKS: u32 = 0x44;
        const TYPE_SLOT: u32 = 0x0c;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let flags = read_u32(this + TASK_FLAGS);
        if flags & 4 == 0 || flags & 2 != 0 {
            return 0;
        }
        let manager = read_u32(context + CONTEXT_MANAGER) + MANAGER_TASKS;
        if lf_checker_rt::callee_thiscall!(1, u32, manager, 1) == 0 {
            return 0;
        }
        let task_lookup = lf_checker_rt::callee_thiscall!(2, u32, manager, 1);

        let child = read_u32(this + CHILD_OBJECT);
        let child_vtable = read_u32(child);
        let child_method = read_u32(child_vtable + TYPE_SLOT);
        let child_type: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(child_method as usize);
        let child_kind = child_type(child);

        let lookup_vtable = read_u32(task_lookup);
        let lookup_method = read_u32(lookup_vtable + TYPE_SLOT);
        let lookup_type: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(lookup_method as usize);
        let lookup_kind = lookup_type(task_lookup);
        if child_kind == lookup_kind {
            return lf_checker_rt::callee_thiscall!(5, u32, manager, 1);
        }
        0
    }
});
