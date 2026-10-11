// original: 0x00A34150 CEntity::mark_pool_entry_active

/// Clears bit zero and sets bit six in the entity flags at +0x24. It then
/// sign-extends the pool index word at +0x2e, selects that word from the
/// global pool-pointer table, and sets bit five in the selected pool flags
/// at +0x40. The selected pool pointer remains in EAX and is returned. This
/// is a no-argument thiscall method.
lf_checker_rt::export!(thiscall, rw_00a34150(this: u32) -> u32 {
    const ENTITY_FLAGS: u32 = 0x24;
    const ENTITY_POOL_INDEX: u32 = 0x2e;
    const POOL_FLAGS: u32 = 0x40;
    const POOL_POINTER_TABLE: u32 = 0x0129_5cd8;

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let entity_flags = read_u32(this + ENTITY_FLAGS);
        write_u32(this + ENTITY_FLAGS, (entity_flags & !1) | 0x40);

        let pool_index = read_u16(this + ENTITY_POOL_INDEX) as i16;
        let table_base = lf_checker_rt::global::<u32>(POOL_POINTER_TABLE) as u32;
        let table_slot = table_base.wrapping_add((i32::from(pool_index) * 4) as u32);
        let pool = read_u32(table_slot);
        let pool_flags = read_u32(pool + POOL_FLAGS);
        write_u32(pool + POOL_FLAGS, pool_flags | 0x20);
        pool
    }
});
