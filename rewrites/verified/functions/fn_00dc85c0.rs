// original: 0x00dc85c0 task_destroy_entity_reference
/// Detach the task's optional entity reference and then run the base task
/// destructor. The entity pointer is stored at `+0x40`; the release helper
/// receives the entity in ECX and the slot address as its stack argument. The
/// class vtable is updated before release. The final thiscall tail target
/// supplies this routine's return value; there are no caller stack arguments.
lf_checker_rt::export!(thiscall, rw_00dc85c0(this_ptr: u32) -> u32 {
    unsafe {
        const CLASS_VTABLE_VA: u32 = 0x00ef4d6c;
        const ENTITY_SLOT: u32 = 0x40;
        const RELEASE_REFERENCE: u32 = 1;
        const BASE_DTOR: u32 = 2;

        let entity = (this_ptr.wrapping_add(ENTITY_SLOT) as *const u32).read_unaligned();
        let slot_ptr = this_ptr.wrapping_add(ENTITY_SLOT);
        (this_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(CLASS_VTABLE_VA));
        if entity != 0 {
            let _released = lf_checker_rt::callee_thiscall!(RELEASE_REFERENCE, u32, entity, slot_ptr);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this_ptr)
    }
});
