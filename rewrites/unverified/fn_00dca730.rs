// original: 0x00DCA730 CTaskComplexCarReactToVehicleCollisionGetOut::vf20

/// Advances or controls the active get-out collision subtask.
///
/// `this` is the task in ECX and `context` is its one stack argument. Task
/// variant seven asks the collision helper and invokes the control slot only
/// when its low-byte result is zero. Variant four checks the subtask and its
/// entity type, asks the ped helper for a result, tests its flag at `+0xd8`,
/// and invokes the control slot only when the returned speed exceeds the
/// stored threshold. Every non-control exit returns the current subtask
/// pointer; a control-slot exit returns that callee's EAX value.
lf_checker_rt::export!(thiscall, rw_00dca730(this: u32, context: u32) -> u32 {
    unsafe {
        const VARIANT_FOUR: u32 = 4;
        const VARIANT_SEVEN: u32 = 7;
        const EXPECTED_SUBTASK_TYPE: u32 = 0x11d;
        const EXPECTED_ENTITY_TYPE: u32 = 0x3ae;
        const VARIANT: u32 = 0x14;
        const CURRENT_SUBTASK: u32 = 0x08;
        const ENTITY_OR_FLAG: u32 = 0x14;
        const HELPER_FLAG: u32 = 0x20;
        const PED_FLAG: u32 = 0xd8;
        const PED_FLAG_MASK: u32 = 1 << 9;
        const SPEED_THRESHOLD_FILE_VA: u32 = 0x00FE8AD8;
        const TYPE_OF_SUBTASK: u32 = 1;
        const TYPE_OF_ENTITY: u32 = 2;
        const FIND_PED: u32 = 3;
        const GET_PED_SPEED: u32 = 4;
        const ALLOW_VARIANT_SEVEN: u32 = 5;
        const CONTROL_SUBTASK: u32 = 6;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let subtask = read_u32(this.wrapping_add(CURRENT_SUBTASK));
        let variant = read_u32(this.wrapping_add(VARIANT));
        if variant == VARIANT_SEVEN {
            let helper_flag = read_u32(this.wrapping_add(HELPER_FLAG));
            let allowed = lf_checker_rt::callee_cdecl!(
                ALLOW_VARIANT_SEVEN,
                u32,
                context,
                helper_flag
            );
            if allowed as u8 != 0 {
                return subtask;
            }
            return lf_checker_rt::callee_thiscall!(CONTROL_SUBTASK, u32, this, context);
        }

        if variant != VARIANT_FOUR || subtask == 0 {
            return subtask;
        }

        let subtask_type = lf_checker_rt::callee_thiscall!(TYPE_OF_SUBTASK, u32, subtask);
        if subtask_type != EXPECTED_SUBTASK_TYPE {
            return subtask;
        }

        let entity = read_u32(subtask.wrapping_add(ENTITY_OR_FLAG));
        if entity == 0 {
            return subtask;
        }
        let entity_type = lf_checker_rt::callee_thiscall!(TYPE_OF_ENTITY, u32, entity);
        if entity_type != EXPECTED_ENTITY_TYPE {
            return subtask;
        }

        let ped = lf_checker_rt::callee_thiscall!(FIND_PED, u32, subtask, context);
        if ped == 0 || read_u32(ped.wrapping_add(PED_FLAG)) & PED_FLAG_MASK == 0 {
            return subtask;
        }

        let speed = lf_checker_rt::callee_thiscall!(GET_PED_SPEED, f32, ped);
        let threshold = lf_checker_rt::global::<f32>(SPEED_THRESHOLD_FILE_VA).read_unaligned();
        if speed > threshold {
            return lf_checker_rt::callee_thiscall!(CONTROL_SUBTASK, u32, this, context);
        }
        subtask
    }
});
