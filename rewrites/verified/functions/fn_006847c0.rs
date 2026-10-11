// original: 0x006847C0 create_frame_record_for_kind

/// Allocate a 32-byte frame record for the low byte of the ECX kind. Kinds
/// zero through three select distinct record tags and vtables; all other
/// kinds return null without allocating. Allocation uses the current thread's
/// slot-zero allocator with kind 0x10. This fastcall helper consumes ECX and
/// ignores EDX.
lf_checker_rt::export!(fastcall, rw_006847c0(kind_input: u32, _edx: u32) -> u32 {
    unsafe {
        const TLS_SERVICE_SLOT: usize = 0;
        const SERVICE_ALLOCATOR: u32 = 8;
        const ALLOCATOR_VTABLE_SLOT: u32 = 8;
        const RECORD_BYTES: u32 = 0x20;
        const ALLOCATION_KIND: u32 = 0x10;

        let (record_tag, vtable_rva) = match kind_input as u8 {
            0 => (0x10, 0x00A9_5580),
            1 => (0x11, 0x00A9_55BC),
            2 => (0x12, 0x00AC_B95C),
            3 => (0x13, 0x00BE_386C),
            _ => return 0,
        };

        let provider = lf_checker_rt::tls_slot(TLS_SERVICE_SLOT);
        let allocator = crate::read_word(provider.wrapping_add(SERVICE_ALLOCATOR));
        let allocator_vtable = crate::read_word(allocator);
        let allocate_address = crate::read_word(allocator_vtable.wrapping_add(ALLOCATOR_VTABLE_SLOT));
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_address as usize);
        let record = allocate(allocator, RECORD_BYTES, ALLOCATION_KIND, 0);
        if record == 0 {
            return 0;
        }

        crate::write_byte(record.wrapping_add(4), record_tag);
        crate::write_word(record, lf_checker_rt::xbase().wrapping_add(vtable_rva));
        record
    }
});
