// original: 0x00dc85f0 task_destroy_entity_reference_with_secondary_base
/// Detach the optional entity reference stored at `+0x40`, set both class
/// vtable fields, then forward to the base destructor. The release helper gets
/// the entity through ECX and receives the slot address as its stack argument;
/// its scripted write clears that slot. The final thiscall tail target returns
/// the result, and this method has no stack arguments of its own.
lf_checker_rt::export!(thiscall, rw_00dc85f0(this_ptr: u32) -> u32 {
    unsafe {
        const PRIMARY_VTABLE_VA: u32 = 0x00ef4eac;
        const SECONDARY_VTABLE_VA: u32 = 0x00ef4f04;
        const ENTITY_SLOT: u32 = 0x40;
        const RELEASE_REFERENCE: u32 = 1;
        const BASE_DTOR: u32 = 2;
        let entity = (this_ptr.wrapping_add(ENTITY_SLOT) as *const u32).read_unaligned();
        let slot_ptr = this_ptr.wrapping_add(ENTITY_SLOT);
        (this_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(PRIMARY_VTABLE_VA));
        (this_ptr.wrapping_add(0x14) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(SECONDARY_VTABLE_VA));
        if entity != 0 {
            let _released = lf_checker_rt::callee_thiscall!(RELEASE_REFERENCE, u32, entity, slot_ptr);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this_ptr)
    }
});
