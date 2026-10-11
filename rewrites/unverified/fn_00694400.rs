// original: 0x00694400 rage::crCreatureComponentBlendShapes::vf2

/// Release and clear the owned resources in a blend-shape component.
///
/// The component is in ECX. Its `+0x18` field is passed in ECX and its `+0x14`
/// field is passed on the stack to the free helper. If `+0x0c` is nonnull, it
/// is released through TLS owner's allocator-manager vtable slot `+0x0c`.
/// The fields at `+4`, `+8`, and `+0x0c` through `+0x18` are then cleared.
lf_checker_rt::export!(thiscall, rw_00694400(component: u32) -> u32 {
    unsafe {
        const FIRST_FREE_THIS_OFFSET: u32 = 0x18;
        const FIRST_FREE_ARG_OFFSET: u32 = 0x14;
        const RESOURCE_OFFSET: u32 = 0x0C;
        const OWNER_ALLOCATOR_OFFSET: u32 = 8;
        const MANAGER_VTABLE_SLOT: u32 = 0x0C;
        const TLS_SLOT: usize = 0;

        let free_this = *((component.wrapping_add(FIRST_FREE_THIS_OFFSET)) as *const u32);
        let free_arg = *((component.wrapping_add(FIRST_FREE_ARG_OFFSET)) as *const u32);
        lf_checker_rt::callee_thiscall!(1, u32, free_this, free_arg);

        let resource = *((component.wrapping_add(RESOURCE_OFFSET)) as *const u32);
        if resource != 0 {
            let owner = lf_checker_rt::tls_slot(TLS_SLOT);
            let manager = *((owner.wrapping_add(OWNER_ALLOCATOR_OFFSET)) as *const u32);
            let vtable = *((manager) as *const u32);
            let release_address = *((vtable.wrapping_add(MANAGER_VTABLE_SLOT)) as *const u32);
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(release_address as usize);
            release(manager, resource);
        }

        let component_bytes = component as *mut u8;
        for offset in [4u32, 8, 0x0C, 0x10, 0x14, 0x18] {
            *((component_bytes.add(offset as usize)) as *mut u32) = 0;
        }
        0
    }
});
