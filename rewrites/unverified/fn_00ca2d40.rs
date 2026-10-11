// original: 0x00CA2D40 task_context_weight_adjustment

/// Adjust a task's weight using the task kind, its associated ped, and the
/// entity relation checks.
///
/// `this` stores the related entity at `+0`, the weight at `+4`, and the task
/// kind at `+0x0c`; `ped` is the one stack argument. A base eligibility call
/// can return early. A lookup result of kind 2 selects one of its two float
/// factors using the ped's player byte. Other kinds use the task kind to
/// select a relocated factor, with the signed-positive result of the entity
/// helper enabling the special factors for kinds `0x31` and `0x32`. Kind
/// `0x34` and kinds outside `0x31` through `0x36` leave the weight alone.
/// A live child record can then zero the weight. Finally, when the global
/// mode is zero and both objects pass the kind and flag guards, matching
/// non-sentinel relation ids also zero the weight. The function is a thiscall
/// with one stack pointer argument and has no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00ca2d40(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_ENTITY: u32 = 0x00;
        const TASK_WEIGHT: u32 = 0x04;
        const TASK_KIND: u32 = 0x0c;
        const ENTITY_FLAGS: u32 = 0x28;
        const ENTITY_PLAYER: u32 = 0x219;
        const ENTITY_CHILD: u32 = 0x6c;
        const ENTITY_MANAGER: u32 = 0x224;
        const ENTITY_RELATIONS: u32 = 0x228;
        const ENTITY_KIND_MASK: u32 = 0x3c0;
        const ENTITY_SUBTYPE: u32 = 0xf50;
        const PED_PLAYER: u32 = 0x219;
        const PED_CHILD: u32 = 0x6c;
        const PED_RELATIONS: u32 = 0x228;
        const CHILD_COUNT: u32 = 0x82e;
        const RELATION_ID: u32 = 0x580;
        const MODE_VA: u32 = 0x110e2fc;
        const FACTOR_KIND31_PLAYER: u32 = 0x103fe04;
        const FACTOR_KIND31_OTHER: u32 = 0x103ff1c;
        const FACTOR_KIND32_PLAYER: u32 = 0x103ff20;
        const FACTOR_KIND32_OTHER: u32 = 0x103ff24;
        const FACTOR_KIND33_PLAYER: u32 = 0x103ff28;
        const FACTOR_KIND33_OTHER: u32 = 0x103ff2c;
        const FACTOR_KIND35_PLAYER: u32 = 0x103ff40;
        const FACTOR_KIND35_OTHER: u32 = 0x103ff44;
        const FACTOR_DEFAULT_PLAYER: u32 = 0x103ff48;
        const FACTOR_DEFAULT_OTHER: u32 = 0x103ff4c;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_f32(address: u32, value: f32) {
            unsafe { write_u32(address, value.to_bits()) }
        }

        #[inline(always)]
        fn multiply(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        fn helper_says_positive(value: u32) -> bool {
            (value as i32) > 0
        }

        let base_allows_task: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        if base_allows_task as u8 == 0 {
            return 0;
        }

        let task_kind = read_u32(this.wrapping_add(TASK_KIND));
        let lookup_info: u32 = lf_checker_rt::callee_cdecl!(2, u32, task_kind);
        let info_kind = read_u32(lookup_info.wrapping_add(0x0c));
        let is_player = read_u8(ped.wrapping_add(PED_PLAYER)) != 0;
        let weight_address = this.wrapping_add(TASK_WEIGHT);

        if info_kind == 2 {
            let factor_offset = if is_player { 0xf0 } else { 0xf4 };
            let factor = read_f32(lookup_info.wrapping_add(factor_offset));
            let weight = read_f32(weight_address);
            write_f32(weight_address, multiply(weight, factor));
        } else {
            let entity = read_u32(this.wrapping_add(TASK_ENTITY));
            let mut helper_positive = false;
            if entity != 0
                && read_u32(entity.wrapping_add(ENTITY_FLAGS)) & ENTITY_KIND_MASK == 0x80
            {
                let helper_value: u32 = lf_checker_rt::callee_thiscall!(3, u32, entity);
                helper_positive = helper_says_positive(helper_value)
                    || read_u32(entity.wrapping_add(ENTITY_SUBTYPE)) != 0;
            }

            let factor_address = match task_kind {
                0x31 if helper_positive => {
                    Some(if is_player { FACTOR_KIND31_PLAYER } else { FACTOR_KIND31_OTHER })
                }
                0x31 => Some(if is_player { FACTOR_DEFAULT_PLAYER } else { FACTOR_DEFAULT_OTHER }),
                0x32 if helper_positive => {
                    Some(if is_player { FACTOR_KIND32_PLAYER } else { FACTOR_KIND32_OTHER })
                }
                0x32 => Some(if is_player { FACTOR_DEFAULT_PLAYER } else { FACTOR_DEFAULT_OTHER }),
                0x33 => Some(if is_player { FACTOR_KIND33_PLAYER } else { FACTOR_KIND33_OTHER }),
                0x34 => None,
                0x35 => Some(if is_player { FACTOR_KIND35_PLAYER } else { FACTOR_KIND35_OTHER }),
                0x36 => Some(if is_player { FACTOR_DEFAULT_PLAYER } else { FACTOR_DEFAULT_OTHER }),
                _ => None,
            };
            if let Some(factor_address) = factor_address {
                let factor = read_f32(lf_checker_rt::relocated(factor_address));
                let weight = read_f32(weight_address);
                write_f32(weight_address, multiply(weight, factor));
            }
        }

        if is_player {
            let child = read_u32(ped.wrapping_add(PED_CHILD));
            if child != 0 && (read_u16(child.wrapping_add(CHILD_COUNT)) as i16) > 0 {
                write_u32(weight_address, 0);
            }
        }

        let entity = read_u32(this.wrapping_add(TASK_ENTITY));
        if read_u32(lf_checker_rt::relocated(MODE_VA)) != 0
            || entity == 0
            || task_kind == 0x3a
        {
            return 0;
        }

        let entity_kind = (read_u32(entity.wrapping_add(ENTITY_FLAGS)) >> 6) & 0x0f;
        let related = match entity_kind {
            3 => entity,
            2 => read_u32(entity.wrapping_add(ENTITY_SUBTYPE)),
            _ => 0,
        };
        if related == 0
            || read_u8(related.wrapping_add(ENTITY_PLAYER)) == 0
            || !is_player
            || related == ped
        {
            return 0;
        }

        let entity_relations = read_u32(related.wrapping_add(ENTITY_RELATIONS));
        let ped_relations = read_u32(ped.wrapping_add(PED_RELATIONS));
        let entity_relation = if entity_relations == 0 {
            u32::MAX
        } else {
            read_u32(entity_relations.wrapping_add(RELATION_ID))
        };
        let ped_relation = if ped_relations == 0 {
            u32::MAX
        } else {
            read_u32(ped_relations.wrapping_add(RELATION_ID))
        };
        if entity_relation == ped_relation && entity_relation != u32::MAX {
            write_u32(weight_address, 0);
        }
        0
    }
});
