// original: 0x00a31ac0 NativeImpl_SET_OBJECT_DYNAMIC_3
/// Physics-body slot lookup behind `SET_OBJECT_DYNAMIC`.
///
/// Follows the entity's rigid-body record (`physics_rigid_body_record` at
/// +0x38) to a body index, validates it against the body pool (index must
/// be below the pool count and its flag entry must have neither of the low
/// two bits set), and returns the indexed slot pointer, or null when any
/// check fails. The original also tests the index sign after zero-extending
/// it, which can never be negative; that dead test is not reproduced.
export!(thiscall, rw_00a31ac0(ent: *const u8) -> u32 {
    unsafe {
        const INVALID: u16 = 0xFFFF;
        let body = *(ent.add(0x38) as *const *const u8);
        if body.is_null() {
            return 0;
        }
        let idx = *(body.add(8) as *const u16) as u32;
        if idx == INVALID as u32 {
            return 0;
        }
        let outer = *global::<*const u8>(0x12B9C7C);
        let pool = *(outer.add(4) as *const *const u8);
        let count = *(pool.add(0x26) as *const u16) as u32;
        if idx >= count {
            return 0;
        }
        let flags = *(pool.add(0x70) as *const *const u8);
        if *flags.add(idx as usize * 8 + 4) & 3 != 0 {
            return 0;
        }
        let slots = *(pool.add(0x80) as *const *const u32);
        *slots.add(idx as usize)
    }
});
