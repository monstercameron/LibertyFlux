// original: 0x006949F0 rage::crCreatureComponentExtraDofs::vf0

/// Release an ExtraDofs component's child objects and optionally notify TLS.
///
/// The component is in ECX and the stack argument's low bit selects the TLS
/// notification. Nonnull child pointers at `+0x0c` and `+0x10` are released
/// through vtable slot zero with flag one, then cleared. The state words at
/// `+4` and `+8` are cleared and the ExtraDofs base vtable is installed. When
/// the low bit is set, TLS owner's manager vtable slot `+0x0c` is called with
/// the component pointer. The component pointer is returned.
lf_checker_rt::export!(thiscall, rw_006949F0(component: u32, flags: u32) -> u32 {
    unsafe {
        const FIRST_CHILD_OFFSET: u32 = 0x0C;
        const SECOND_CHILD_OFFSET: u32 = 0x10;
        const OWNER_MANAGER_OFFSET: u32 = 8;
        const MANAGER_VTABLE_SLOT: u32 = 0x0C;
        const TLS_SLOT: usize = 0;
        const DESTROY_FLAG: u32 = 1;
        const BASE_VTABLE: u32 = 0x00FE38AC;

        let component_bytes = component as *mut u8;
        for offset in [FIRST_CHILD_OFFSET, SECOND_CHILD_OFFSET] {
            let child = *((component_bytes.add(offset as usize)) as *const u32);
            if child != 0 {
                let vtable = *((child) as *const u32);
                let destroy_address = *((vtable) as *const u32);
                let destroy: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(destroy_address as usize);
                destroy(child, DESTROY_FLAG);
                *((component_bytes.add(offset as usize)) as *mut u32) = 0;
            }
        }

        *((component_bytes.add(4)) as *mut u32) = 0;
        *((component_bytes.add(8)) as *mut u32) = 0;
        (component_bytes as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_VTABLE));

        if flags & 1 != 0 {
            let owner = lf_checker_rt::tls_slot(TLS_SLOT);
            let manager = *((owner.wrapping_add(OWNER_MANAGER_OFFSET)) as *const u32);
            let vtable = *((manager) as *const u32);
            let notify_address = *((vtable.wrapping_add(MANAGER_VTABLE_SLOT)) as *const u32);
            let notify: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(notify_address as usize);
            notify(manager, component);
        }
        component
    }
});
