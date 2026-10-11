// original: 0x00694CD0 rage::crCreatureComponentExtraDofs::vf2

/// Release the two optional resources held by an ExtraDofs component.
///
/// The component is in ECX. Its resource pointers are at `+0x0c` and `+0x10`;
/// each nonnull resource is released through vtable slot zero with flag one.
/// Both pointer fields and the two state words at `+4` and `+8` are cleared.
/// The call order follows the two fields, and the method has no stack args.
lf_checker_rt::export!(thiscall, rw_00694CD0(component: u32) -> u32 {
    unsafe {
        const FIRST_RESOURCE_OFFSET: u32 = 0x0C;
        const SECOND_RESOURCE_OFFSET: u32 = 0x10;
        const FIRST_STATE_OFFSET: u32 = 4;
        const SECOND_STATE_OFFSET: u32 = 8;
        const RELEASE_VTABLE_SLOT: u32 = 0;
        const RELEASE_FLAG: u32 = 1;

        let component_bytes = component as *mut u8;
        for offset in [FIRST_RESOURCE_OFFSET, SECOND_RESOURCE_OFFSET] {
            let resource = *((component_bytes.add(offset as usize)) as *mut u32);
            if resource != 0 {
                let vtable = *((resource) as *const u32);
                let release_address = *((vtable.wrapping_add(RELEASE_VTABLE_SLOT)) as *const u32);
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(release_address as usize);
                release(resource, RELEASE_FLAG);
                *((component_bytes.add(offset as usize)) as *mut u32) = 0;
            }
        }
        *((component_bytes.add(FIRST_STATE_OFFSET as usize)) as *mut u32) = 0;
        *((component_bytes.add(SECOND_STATE_OFFSET as usize)) as *mut u32) = 0;
        0
    }
});
