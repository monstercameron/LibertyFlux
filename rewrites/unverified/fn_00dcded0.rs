// original: 0x00dcded0 CTaskSimpleReloadGun::SetupReloadAnimation

/// Select and create or find a reload animation for the ped's weapon, then
/// initialize the task's animation times, state, callback, and rate. The
/// animation helper calls are kept explicit so their arguments and order
/// match the original routine. The task rate calculation uses the same
/// single-precision operation order as the original, including the global
/// scale values and the ped's per-weapon byte.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00DCDED0(task_ptr: u32, ped_ptr: u32) -> u32 {
    const TASK_ID_A: u32 = 0x4C;
    const TASK_KIND_A: u32 = 0x50;
    const TASK_ID_B: u32 = 0x54;
    const TASK_KIND_B: u32 = 0x58;
    const TASK_RATE: u32 = 0x5C;
    const TASK_ANIMATION: u32 = 0x64;
    const TASK_TIME_START: u32 = 0x6C;
    const TASK_TIME_MIDDLE: u32 = 0x70;
    const TASK_TIME_END: u32 = 0x78;
    const TASK_RATE_START: u32 = 0x44;
    const TASK_RATE_END: u32 = 0x48;
    const TASK_FLAGS: u32 = 0x8C;
    const TASK_STATE: u32 = 0x90;
    const TASK_STATE_AUX: u32 = 0x98;
    const TASK_RATE_RESULT: u32 = 0x40;
    const TASK_PREVIOUS_STATE: u32 = 0x94;
    const PED_OWNER: u32 = 0x78;
    const PED_WEAPON_RATE: u32 = 0x370;
    const PED_FLAGS: u32 = 0x26C;
    const PED_WEAPON_MANAGER: u32 = 0x2B0;
    const MANAGER_WEAPON_SLOT: u32 = 0x18;
    const INFO_FLAGS: u32 = 0x20;
    const INFO_TYPE: u32 = 0x08;
    const INFO_ANIMATION_ID: u32 = 0x28;
    const ANIMATION_FLAGS: u32 = 0x04;
    const ANIMATION_COMPONENT: u32 = 0x40;
    const ANIMATION_TIME: u32 = 0x4C;
    const ANIMATION_FIRST_TIME: u32 = 0x64;
    const ANIMATION_SECOND_TIME: u32 = 0x68;
    const ANIMATION_THIRD_TIME: u32 = 0x6C;
    const ANIMATION_FOURTH_TIME: u32 = 0x70;
    const PED_STATE_CONDITION: u32 = 3;
    const ANIMATION_KIND_PRIMARY: u32 = 0xC5;
    const ANIMATION_KIND_ALTERNATE: u32 = 0xC6;
    const ANIMATION_CALLBACK: u32 = 0x00DC_DD00;
    const ANIMATION_REFRESH: u32 = 2;
    const ANIMATION_FLAGS_NORMALIZED: u32 = 0x0000_0010 | 0x0000_8000 | 0x0000_4000;
    const ANIMATION_FLAGS_CLEAR: u32 = 0x40 | 0x80;
    const ANIMATION_FLAGS_TASK_BIT: u32 = 0x0000_0001 | 0x0020_0000;
    const PED_FLAGS_RELOADING: u32 = 0x0004_0000;
    const TASK_FLAG_STATE_OVERRIDE: u32 = 0x08;
    const TASK_FLAG_RATE_UPDATE: u32 = 0x0010_0000;
    const TASK_FLAG_ANIMATION_ACTIVE: u32 = 0x80;

    const GLOBAL_TIME_ONE: u32 = 0x00FE_88E8;
    const GLOBAL_RATE_SCALE: u32 = 0x00FE_8684;
    const GLOBAL_BASE_RATE: u32 = 0x0105_7AAC;
    const GLOBAL_WEAPON_SCALE: u32 = 0x00FE_8A24;
    const GLOBAL_PED_SCALE: u32 = 0x00FE_8830;
    const GLOBAL_WEAPON_FACTOR: u32 = 0x00FE_8764;
    const GLOBAL_RATE_BIAS: u32 = 0x00FE_888C;

    const C_MANAGER: u32 = 1;
    const C_WEAPON_INFO: u32 = 2;
    const C_PED_CONDITION: u32 = 3;
    const C_FIND_ANIMATION: u32 = 4;
    const C_CREATE_ANIMATION: u32 = 5;
    const C_REFRESH_COMPONENT: u32 = 6;
    const C_SET_TIME: u32 = 7;
    const C_SET_RATE: u32 = 8;
    const C_ATTACH_CALLBACK: u32 = 9;
    const C_RANDOM: u32 = 10;

    #[inline(always)]
    fn read_word(address: u32) -> u32 {
        // SAFETY: contract inputs map every object field read by this routine.
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    fn write_word(address: u32, value: u32) {
        // SAFETY: task, ped, and animation fields are mapped mutable inputs.
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    fn read_byte(address: u32) -> u8 {
        // SAFETY: the ped fixture maps the byte at its weapon-rate field.
        unsafe { (address as *const u8).read_unaligned() }
    }

    #[inline(always)]
    fn read_float(address: u32) -> f32 {
        f32::from_bits(read_word(address))
    }

    #[inline(always)]
    fn write_float(address: u32, value: f32) {
        write_word(address, value.to_bits());
    }

    #[inline(always)]
    fn global_float(file_va: u32) -> f32 {
        let pointer = lf_checker_rt::global::<u32>(file_va);
        f32::from_bits(unsafe { pointer.read_unaligned() })
    }

    #[inline(always)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    #[inline(always)]
    fn subtract(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }

    #[inline(always)]
    fn multiply(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    #[inline(always)]
    fn divide(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) / core::hint::black_box(right)
    }

    let weapon_manager = lf_checker_rt::callee_thiscall!(
        C_MANAGER,
        u32,
        ped_ptr.wrapping_add(PED_WEAPON_MANAGER),
    );
    let weapon_slot = read_word(weapon_manager.wrapping_add(MANAGER_WEAPON_SLOT));
    let weapon_info = lf_checker_rt::callee_cdecl!(C_WEAPON_INFO, u32, weapon_slot);

    let uses_alternate_pair = (lf_checker_rt::callee_thiscall!(
        C_PED_CONDITION,
        u32,
        ped_ptr,
    ) & 0xFF)
        != 0;

    let (mut animation_id, mut animation_kind) = if uses_alternate_pair {
        (read_word(task_ptr.wrapping_add(TASK_ID_B)), read_word(task_ptr.wrapping_add(TASK_KIND_B)))
    } else {
        (read_word(task_ptr.wrapping_add(TASK_ID_A)), read_word(task_ptr.wrapping_add(TASK_KIND_A)))
    };

    if animation_id == u32::MAX || animation_kind == u32::MAX {
        animation_id = read_word(weapon_info.wrapping_add(INFO_ANIMATION_ID));
        if uses_alternate_pair
            && animation_id != u32::MAX
            && (read_word(weapon_info.wrapping_add(INFO_FLAGS)) & 0x8000) != 0
        {
            animation_kind = ANIMATION_KIND_ALTERNATE;
        } else {
            animation_kind = ANIMATION_KIND_PRIMARY;
        }
    }

    let owner = read_word(ped_ptr.wrapping_add(PED_OWNER));
    let mut animation = lf_checker_rt::callee_thiscall!(
        C_FIND_ANIMATION,
        u32,
        owner,
        animation_id,
        animation_kind,
    );
    if animation == 0 {
        let initial_rate = read_word(task_ptr.wrapping_add(TASK_RATE));
        animation = lf_checker_rt::callee_thiscall!(
            C_CREATE_ANIMATION,
            u32,
            owner,
            animation_id,
            animation_kind,
            initial_rate,
            u32::MAX,
        );
    }
    write_word(task_ptr.wrapping_add(TASK_ANIMATION), animation);

    let component = read_word(animation.wrapping_add(ANIMATION_COMPONENT));
    let _ = lf_checker_rt::callee_thiscall!(C_REFRESH_COMPONENT, u32, animation, component);
    write_word(
        task_ptr.wrapping_add(TASK_TIME_START),
        read_word(animation.wrapping_add(ANIMATION_FIRST_TIME)),
    );
    let _ = lf_checker_rt::callee_thiscall!(C_REFRESH_COMPONENT, u32, animation, component);
    write_word(
        task_ptr.wrapping_add(TASK_TIME_MIDDLE),
        read_word(animation.wrapping_add(ANIMATION_SECOND_TIME)),
    );
    let _ = lf_checker_rt::callee_thiscall!(C_REFRESH_COMPONENT, u32, animation, component);
    write_word(
        task_ptr.wrapping_add(0x74),
        read_word(animation.wrapping_add(ANIMATION_THIRD_TIME)),
    );
    let _ = lf_checker_rt::callee_thiscall!(C_REFRESH_COMPONENT, u32, animation, component);
    write_word(
        task_ptr.wrapping_add(TASK_TIME_END),
        read_word(animation.wrapping_add(ANIMATION_FOURTH_TIME)),
    );

    let current_time = read_float(animation.wrapping_add(ANIMATION_TIME));
    let _ = lf_checker_rt::callee_thiscall!(C_SET_TIME, u32, animation, 0_u32);
    let start_time = read_float(task_ptr.wrapping_add(TASK_TIME_START));
    let middle_time = read_float(task_ptr.wrapping_add(TASK_TIME_MIDDLE));
    let end_time = read_float(task_ptr.wrapping_add(TASK_TIME_END));

    if start_time > current_time {
        let _ = lf_checker_rt::callee_thiscall!(
            C_SET_TIME,
            u32,
            animation,
            current_time.to_bits(),
        );
    } else if middle_time > current_time || end_time > current_time {
        let previous_state = read_word(task_ptr.wrapping_add(TASK_PREVIOUS_STATE));
        let state = if previous_state == PED_STATE_CONDITION {
            PED_STATE_CONDITION
        } else if previous_state == 5 {
            5
        } else {
            7
        };
        write_word(task_ptr.wrapping_add(TASK_STATE), state);
    } else {
        let time_one = global_float(GLOBAL_TIME_ONE);
        let numerator = subtract(time_one, current_time);
        let denominator = subtract(time_one, end_time);
        let ratio = divide(numerator, denominator);
        let time_value = multiply(ratio, start_time);
        let _ = lf_checker_rt::callee_thiscall!(
            C_SET_TIME,
            u32,
            animation,
            time_value.to_bits(),
        );
    }

    let task_flags = read_word(task_ptr.wrapping_add(TASK_FLAGS));
    if (task_flags & TASK_FLAG_STATE_OVERRIDE) != 0 {
        write_word(task_ptr.wrapping_add(TASK_STATE), PED_STATE_CONDITION);
        write_word(task_ptr.wrapping_add(TASK_STATE_AUX), 7);
    }

    let callback = lf_checker_rt::relocated(ANIMATION_CALLBACK);
    let _ = lf_checker_rt::callee_thiscall!(
        C_ATTACH_CALLBACK,
        u32,
        animation,
        ANIMATION_REFRESH,
        callback,
        task_ptr,
    );
    let rate = read_word(task_ptr.wrapping_add(TASK_RATE));
    let _ = lf_checker_rt::callee_thiscall!(C_SET_RATE, u32, animation, rate);

    let mut animation_flags = read_word(animation.wrapping_add(ANIMATION_FLAGS));
    animation_flags &= !ANIMATION_FLAGS_CLEAR;
    animation_flags |= ANIMATION_FLAGS_NORMALIZED;

    if (task_flags & TASK_FLAG_RATE_UPDATE) != 0 {
        let initial_rate = read_float(task_ptr.wrapping_add(TASK_RATE_START));
        let target_rate = read_float(task_ptr.wrapping_add(TASK_RATE_END));
        if initial_rate > 0.0 && target_rate > 0.0 {
            let random_value = lf_checker_rt::callee_cdecl!(C_RANDOM, u32,);
            let random_float = (random_value as i32) as f32;
            let scaled_random = multiply(random_float, global_float(GLOBAL_RATE_SCALE));
            let rate_range = subtract(target_rate, initial_rate);
            let weighted_range = multiply(scaled_random, rate_range);
            write_float(
                task_ptr.wrapping_add(TASK_RATE_RESULT),
                add(weighted_range, initial_rate),
            );
        } else {
            let mut result_rate = global_float(GLOBAL_BASE_RATE);
            write_float(task_ptr.wrapping_add(TASK_RATE_RESULT), result_rate);
            let second_info = lf_checker_rt::callee_cdecl!(C_WEAPON_INFO, u32, weapon_slot);
            if read_word(second_info.wrapping_add(INFO_TYPE)) == PED_STATE_CONDITION {
                result_rate = multiply(result_rate, global_float(GLOBAL_WEAPON_SCALE));
                write_float(task_ptr.wrapping_add(TASK_RATE_RESULT), result_rate);
            }

            let random_value = lf_checker_rt::callee_cdecl!(C_RANDOM, u32,);
            let random_float = (random_value as i32) as f32;
            let random_scaled = multiply(random_float, global_float(GLOBAL_RATE_SCALE));
            let ped_scaled_random = multiply(random_scaled, global_float(GLOBAL_PED_SCALE));
            let weapon_rate = read_byte(ped_ptr.wrapping_add(PED_WEAPON_RATE)) as f32;
            let weapon_rate_scaled = multiply(weapon_rate, global_float(GLOBAL_WEAPON_FACTOR));
            let rate_with_bias = add(ped_scaled_random, global_float(GLOBAL_RATE_BIAS));
            let numerator = multiply(rate_with_bias, result_rate);
            write_float(
                task_ptr.wrapping_add(TASK_RATE_RESULT),
                divide(numerator, weapon_rate_scaled),
            );
        }
    }

    if (task_flags & TASK_FLAG_ANIMATION_ACTIVE) != 0 {
        animation_flags |= ANIMATION_FLAGS_TASK_BIT;
    }
    write_word(animation.wrapping_add(ANIMATION_FLAGS), animation_flags);
    write_word(
        ped_ptr.wrapping_add(PED_FLAGS),
        read_word(ped_ptr.wrapping_add(PED_FLAGS)) | PED_FLAGS_RELOADING,
    );

    if (task_flags & TASK_FLAG_ANIMATION_ACTIVE) != 0 {
        animation
    } else {
        task_flags >> 7
    }
}
