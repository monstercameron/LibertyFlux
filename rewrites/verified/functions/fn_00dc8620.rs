// original: 0x00dc8620 task_destroy_entity_and_target
/// Clear the task's optional entity reference at `+0x70`, remove its optional
/// target at `+0x74` through the manager pointer stored in a global word, set
/// both intermediate vtables, and then invoke the base destructor. The entity
/// release helper clears the entity slot; the target-removal helper is followed
/// by an explicit zero store. The base destructor is a thiscall tail target.
lf_checker_rt::export!(thiscall, rw_00dc8620(this_ptr: u32) -> u32 {
    unsafe {
        const PRIMARY_VTABLE_VA: u32 = 0x00ef4e1c;
        const SECONDARY_VTABLE_VA: u32 = 0x00ef4e74;
        const ENTITY_SLOT: u32 = 0x70;
        const TARGET_SLOT: u32 = 0x74;
        const MANAGER_GLOBAL_VA: u32 = 0x0179d114;
        const RELEASE_REFERENCE: u32 = 1;
        const REMOVE_TARGET: u32 = 2;
        const BASE_DTOR: u32 = 3;

        let entity_slot = this_ptr.wrapping_add(ENTITY_SLOT);
        let entity = (entity_slot as *const u32).read_unaligned();
        (this_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(PRIMARY_VTABLE_VA));
        (this_ptr.wrapping_add(0x14) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(SECONDARY_VTABLE_VA));
        if entity != 0 {
            let _released = lf_checker_rt::callee_thiscall!(RELEASE_REFERENCE, u32, entity, entity_slot);
        }

        let target_slot = this_ptr.wrapping_add(TARGET_SLOT);
        let target = (target_slot as *const u32).read_unaligned();
        if target != 0 {
            let manager = (lf_checker_rt::relocated(MANAGER_GLOBAL_VA) as *const u32).read_unaligned();
            let _removed = lf_checker_rt::callee_thiscall!(REMOVE_TARGET, u32, manager, target);
            (target_slot as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this_ptr)
    }
});
