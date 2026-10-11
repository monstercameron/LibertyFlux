// original: 0x00A34530 CEntity::has_pool_sentinel

/// Tests a 16-bit sentinel field selected by the entity type in flags at
/// +0x28. Type 4 reads the word at +0x220, type 2 reads the word at +0x1074,
/// and all other types return false. The result is the exact 32-bit boolean
/// value zero or one. This is a no-argument thiscall method.
lf_checker_rt::export!(thiscall, rw_00a34530(this: u32) -> u32 {
    const FLAGS: u32 = 0x28;
    const TYPE_SHIFT: u32 = 6;
    const TYPE_MASK: u32 = 0x0f;
    const TYPE_A: u32 = 4;
    const TYPE_B: u32 = 2;
    const TYPE_A_SENTINEL: u32 = 0x220;
    const TYPE_B_SENTINEL: u32 = 0x1074;
    const SENTINEL: u16 = 0xfffe;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    unsafe {
        let entity_type = (read_u32(this + FLAGS) >> TYPE_SHIFT) & TYPE_MASK;
        if entity_type == TYPE_A {
            u32::from(read_u16(this + TYPE_A_SENTINEL) == SENTINEL)
        } else if entity_type == TYPE_B {
            u32::from(read_u16(this + TYPE_B_SENTINEL) == SENTINEL)
        } else {
            0
        }
    }
});
