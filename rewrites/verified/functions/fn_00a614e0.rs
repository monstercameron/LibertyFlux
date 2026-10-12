// original: 0x00A614E0 entity_type4_flag_test

/// Returns one in AL when the entity's type bits at +0x28 equal 0x100 within
/// mask 0x3c0, its activity byte at +0x22b is nonzero, and bit two of the
/// word at +0x24 is clear. All three tests are required. The entity pointer is
/// the first cdecl stack argument; EAX's low byte is the Boolean result.
lf_checker_rt::export!(cdecl, rw_00A614E0(entity: u32) -> u32 {
    const TYPE_FIELD: u32 = 0x28;
    const TYPE_MASK: u32 = 0x3c0;
    const REQUIRED_TYPE: u32 = 0x100;
    const ACTIVE_BYTE: u32 = 0x22b;
    const STATE_FLAGS: u32 = 0x24;
    const BLOCKED_BIT: u32 = 1 << 2;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    let type_flags = unsafe { read_u32(entity.wrapping_add(TYPE_FIELD)) };
    let activity = unsafe { read_u8(entity.wrapping_add(ACTIVE_BYTE)) };
    let state_flags = unsafe { read_u32(entity.wrapping_add(STATE_FLAGS)) };
    u32::from(type_flags & TYPE_MASK == REQUIRED_TYPE
        && activity != 0
        && state_flags & BLOCKED_BIT == 0)
});
