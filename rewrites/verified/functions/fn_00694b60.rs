// original: 0x00694B60 rage::crCreatureComponentExtraDofs::vf11

/// Allocate and clone an extra-DOF component.
///
/// The source object is in ECX. TLS slot zero leads to an owner with its
/// allocator pointer at `+8`; the allocator's vtable slot `+8` receives a
/// `0x14`-byte allocation request tagged `0x10`. A null allocation returns
/// null. Otherwise the destination receives the ExtraDofs vtable and the four
/// source words at offsets `+4` through `+0x10`, then its pointer is returned.
lf_checker_rt::export!(thiscall, rw_00694B60(source: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const OWNER_ALLOCATOR_OFFSET: u32 = 8;
        const ALLOCATOR_VTABLE_SLOT: u32 = 8;
        const BLOCK_SIZE: u32 = 0x14;
        const ALLOCATION_TAG: u32 = 0x10;
        const EXTRA_DOFS_VTABLE: u32 = 0x00FE3910;

        let owner = lf_checker_rt::tls_slot(TLS_SLOT);
        let allocator = *((owner.wrapping_add(OWNER_ALLOCATOR_OFFSET)) as *const u32);
        let vtable = *((allocator) as *const u32);
        let allocate_address = *((vtable.wrapping_add(ALLOCATOR_VTABLE_SLOT)) as *const u32);
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_address as usize);
        let clone = allocate(allocator, BLOCK_SIZE, ALLOCATION_TAG, 0);
        if clone == 0 {
            return 0;
        }

        let destination_bytes = clone as *mut u8;
        let source_bytes = source as *const u8;
        (destination_bytes as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(EXTRA_DOFS_VTABLE));
        let mut field_offset = 4;
        while field_offset <= 0x10 {
            *((destination_bytes.add(field_offset as usize)) as *mut u32) =
                *((source_bytes.add(field_offset as usize)) as *const u32);
            field_offset += 4;
        }
        clone
    }
});
