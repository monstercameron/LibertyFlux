// original: 0x00A33F80 CEntity::vf14

/// Updates entity flags from a pool entry selected by the stack argument.
/// The pool table has one 32-bit pointer per index; the selected entry has a
/// vtable at +0, feature flags at +0x40, a state byte at +0x5b, a threshold
/// float at +0x1c, a value at +0x2c, and a sentinel word at +0x52. Three
/// helpers and one virtual query supply scripted results. The helper results
/// tested with `jg` are signed 32-bit values; the pool index and the global
/// index comparison are unsigned 32-bit values. This is a thiscall method
/// with one stack argument; EAX returns the selected entry's value.
lf_checker_rt::export!(thiscall, rw_00a33f80(this: u32, pool_index: u32) -> u32 {
    const ENTITY_FLAGS: u32 = 0x28;
    const ENTITY_STATE: u32 = 0x24;
    const ENTITY_MODEL_INDEX: u32 = 0x2e;
    const ENTITY_POOL_VALUE: u32 = 0x50;
    const POOL_TABLE: u32 = 0x1295_cd8;
    const GLOBAL_MODEL_INDEX: u32 = 0x12bd_0f4;
    const GLOBAL_THRESHOLD: u32 = 0x00fe_8830;
    const POOL_VTABLE_SLOT_QUERY: u32 = 0x0c;
    const POOL_FEATURES: u32 = 0x40;
    const POOL_STATE: u32 = 0x5b;
    const POOL_THRESHOLD: u32 = 0x1c;
    const POOL_VALUE: u32 = 0x2c;
    const POOL_SENTINEL: u32 = 0x52;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u16(address: u32, value: u16) {
        unsafe { (address as *mut u16).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let pool_table = lf_checker_rt::global::<u32>(POOL_TABLE) as u32;
        let pool = read_u32(pool_table.wrapping_add(pool_index.wrapping_mul(4)));
        write_u16(this + ENTITY_MODEL_INDEX, pool_index as u16);

        let mut entity_flags = read_u32(this + ENTITY_FLAGS);
        if entity_flags & 0x10 == 0 {
            let enabled = lf_checker_rt::callee_thiscall!(1, u32, this) as u8;
            entity_flags = (entity_flags & !0x10) | (u32::from(enabled & 1) << 4);
            write_u32(this + ENTITY_FLAGS, entity_flags);
        }

        if read_u32(pool + POOL_FEATURES) & 0x2000 != 0 {
            entity_flags |= 0x0400_0000;
            write_u32(this + ENTITY_FLAGS, entity_flags);
        }

        let signed_count = lf_checker_rt::callee_thiscall!(2, u32, pool);
        if (signed_count as i32) > 0 {
            let state_flags = read_u32(this + ENTITY_STATE);
            write_u32(this + ENTITY_STATE, state_flags | 0x0080_0000);
        } else if read_u8(pool + POOL_STATE) & 0x40 == 0 {
            let signed_extra = lf_checker_rt::callee_thiscall!(3, u32, pool, 0x12);
            if (signed_extra as i32) > 0 {
                let state_flags = read_u32(this + ENTITY_STATE);
                write_u32(this + ENTITY_STATE, state_flags | 0x0080_0000);
            }
        }

        let global_threshold = f32::from_bits(read_u32(
            lf_checker_rt::global::<u32>(GLOBAL_THRESHOLD) as u32,
        ));
        let pool_threshold = f32::from_bits(read_u32(pool + POOL_THRESHOLD));
        if global_threshold > pool_threshold || entity_flags & 0x3c0 == 0x40 {
            let state_flags = read_u32(this + ENTITY_STATE);
            write_u32(this + ENTITY_STATE, state_flags | 0x4000_0000);
        }

        let vtable = read_u32(pool);
        let query_address = read_u32(vtable + POOL_VTABLE_SLOT_QUERY);
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(query_address as usize);
        let query_result = query(pool) as u8;
        let global_index = read_u32(
            lf_checker_rt::global::<u32>(GLOBAL_MODEL_INDEX) as u32,
        );
        if query_result == 2 || pool_index == global_index {
            entity_flags |= 0x8000;
        } else {
            entity_flags &= !0x8000;
        }

        let pool_value = read_u32(pool + POOL_VALUE);
        write_u32(this + ENTITY_POOL_VALUE, pool_value);
        if read_u16(pool + POOL_SENTINEL) != u16::MAX {
            entity_flags |= 0x0800_0000;
        }
        write_u32(this + ENTITY_FLAGS, entity_flags);
        pool_value
    }
});
