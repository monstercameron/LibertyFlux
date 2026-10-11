// original: 0x00694200 rage::crCreatureComponentBlendShapes::vf11

/// Clone a blend-shape component through the thread-local allocator.
///
/// TLS slot zero points to an owner whose allocator pointer is at `+8`. The
/// allocator's vtable slot `+8` receives a request for a `0x30`-byte block
/// with tag `0x10`; a null answer returns null. Otherwise the copy initializer
/// is called with the new block as ECX and the source component as its stack
/// argument. Its result is returned. The method takes its source in ECX and
/// has no stack arguments.
lf_checker_rt::export!(thiscall, rw_00694200(source: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const ALLOCATOR_OFFSET: u32 = 8;
        const ALLOCATOR_VTABLE_SLOT: u32 = 8;
        const BLOCK_SIZE: u32 = 0x30;
        const ALLOCATION_TAG: u32 = 0x10;
        const COPY_INITIALIZER: u32 = 2;

        let owner = lf_checker_rt::tls_slot(TLS_SLOT);
        let allocator = *((owner.wrapping_add(ALLOCATOR_OFFSET)) as *const u32);
        let vtable = *((allocator) as *const u32);
        let allocate_address = *((vtable.wrapping_add(ALLOCATOR_VTABLE_SLOT)) as *const u32);
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_address as usize);
        let block = allocate(allocator, BLOCK_SIZE, ALLOCATION_TAG, 0);
        if block == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(COPY_INITIALIZER, u32, block, source)
    }
});
