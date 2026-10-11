// original: 0x00684570 rage::crFrameDofInt::vf1

/// Clone the 32-byte frame-DOF record through the current thread's allocator.
/// The clone retains every source field from byte 4 onward and receives the
/// integer-frame vtable. Allocation requests use a 32-byte size and kind 0x10;
/// a null allocation returns null without touching memory. This is a
/// thiscall method with no stack arguments and returns the allocated pointer.
lf_checker_rt::export!(thiscall, rw_00684570(this: u32) -> u32 {
    unsafe {
        const TLS_SERVICE_SLOT: usize = 0;
        const SERVICE_ALLOCATOR: u32 = 8;
        const ALLOCATOR_VTABLE_SLOT: u32 = 8;
        const RECORD_BYTES: u32 = 0x20;
        const ALLOCATION_KIND: u32 = 0x10;
        const INTEGER_FRAME_VTABLE_RVA: u32 = 0x00BE_386C;

        let provider = lf_checker_rt::tls_slot(TLS_SERVICE_SLOT);
        let allocator = crate::read_word(provider.wrapping_add(SERVICE_ALLOCATOR));
        let allocator_vtable = crate::read_word(allocator);
        let allocate_address = crate::read_word(allocator_vtable.wrapping_add(ALLOCATOR_VTABLE_SLOT));
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_address as usize);
        let clone = allocate(allocator, RECORD_BYTES, ALLOCATION_KIND, 0);

        if clone == 0 {
            return 0;
        }

        crate::write_word(
            clone,
            lf_checker_rt::xbase().wrapping_add(INTEGER_FRAME_VTABLE_RVA),
        );
        crate::write_byte(clone + 4, crate::read_byte(this + 4));
        crate::write_byte(clone + 5, crate::read_byte(this + 5));
        crate::write_halfword(clone + 6, crate::read_halfword(this + 6));
        for offset in [8, 12, 16, 20, 24, 28] {
            crate::write_word(clone + offset, crate::read_word(this + offset));
        }
        clone
    }
});
