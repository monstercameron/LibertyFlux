// original: 0x00A619A0 entity_link_active_check

/// Returns one in AL when the entity's state byte at +0x219 is nonzero, its
/// activity byte at +0x26c has bit two set, its link pointer at +0xb30 is
/// non-null, and the linked record's byte at +0xf1c lacks bit five. The input
/// is a cdecl pointer to the entity; the final linked-record test is reached
/// only after the first three conditions hold.
lf_checker_rt::export!(cdecl, rw_00A619A0(entity: u32) -> u32 {
    const ENTITY_STATE: u32 = 0x219;
    const ENTITY_FLAGS: u32 = 0x26c;
    const LINK_POINTER: u32 = 0xb30;
    const LINKED_FLAGS: u32 = 0xf1c;
    const ACTIVE_BIT: u8 = 0x04;
    const BLOCKING_BIT: u8 = 0x20;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    if unsafe { read_u8(entity.wrapping_add(ENTITY_STATE)) } == 0 {
        return 0;
    }
    if unsafe { read_u8(entity.wrapping_add(ENTITY_FLAGS)) } & ACTIVE_BIT == 0 {
        return 0;
    }
    let linked_record = unsafe { read_u32(entity.wrapping_add(LINK_POINTER)) };
    if linked_record == 0 {
        return 0;
    }
    let linked_flags = unsafe { read_u8(linked_record.wrapping_add(LINKED_FLAGS)) };
    u32::from(linked_flags & BLOCKING_BIT == 0)
});
