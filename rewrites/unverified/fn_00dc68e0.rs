// original: 0x00DC68E0 CTaskSimpleFireGun::build_fire_animation

/// Rebuilds the fire animation for a simple gun task.
///
/// `task` is the task object in ECX and `ped` is its one stack argument.
/// The routine obtains weapon animation metadata, selects a pair of animation
/// keys from the task or weapon record, asks the ped's animation manager for
/// an association, and updates that association's timing and flags. The task
/// fields at `+0x40`, `+0x4c`, `+0x68`, `+0x70..+0x7c`, `+0x94`, and `+0x9c`
/// hold the selected key, helper result, association, sampled track data, and
/// playback settings. The animation association fields at `+0x04`, `+0x4c`,
/// and `+0x54` hold flags, clip time, and blend speed. Callee calls are
/// dispatched through the checker runtime; the virtual ped method is planted
/// in the contract's vtable slot. Floating-point operations retain the
/// original operand order, and the SSE truncating conversion returns the
/// processor's integer-indefinite value for NaN or out-of-range input.
lf_checker_rt::export!(thiscall, rw_00dc68e0(task: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_LINK_PRIMARY: u32 = 0x50;
        const TASK_LINK_SECONDARY: u32 = 0x54;
        const TASK_LINK_FIRE: u32 = 0x58;
        const TASK_LINK_FIRE_SECONDARY: u32 = 0x5c;
        const TASK_INITIAL_BLEND: u32 = 0x60;
        const TASK_ASSOCIATION: u32 = 0x68;
        const TASK_TRACK_COUNT: u32 = 0x4c;
        const TASK_TRACK_KEYS: u32 = 0x70;
        const TASK_SELECTED_KEY: u32 = 0x40;
        const TASK_FLAGS: u32 = 0x90;
        const TASK_PLAYBACK_MODE: u32 = 0x94;
        const TASK_PLAYBACK_VARIANT: u32 = 0x98;
        const TASK_RANDOM_MIN: u32 = 0x42;
        const TASK_RANDOM_MAX: u32 = 0x44;
        const TASK_RANDOM_VARIANT: u32 = 0x9c;
        const PED_ANIMATION_MANAGER: u32 = 0x78;
        const PED_WEAPON_INFO: u32 = 0x2b0;
        const PED_SCALE_DATA: u32 = 0x228;
        const PED_ANIMATION_LIMIT: u32 = 0x5c;
        const PED_ANIMATION_RATE: u32 = 0x27c;
        const PED_ANIMATION_RATE_BYTE: u32 = 0x370;
        const SCALE_DATA_FIRE_RATE: u32 = 0x5a8;
        const INFO_FLAGS: u32 = 0x20;
        const INFO_PRIMARY_KEY: u32 = 0x28;
        const INFO_STANDARD_RATE: u32 = 0x2c;
        const INFO_ALTERNATE_RATE: u32 = 0x30;
        const INFO_ANIMATION_COUNT: u32 = 0x86;
        const INFO_DIVISOR: u32 = 0x8c;
        const MANAGER_INFO_KEY: u32 = 0x18;
        const ASSOCIATION_FLAGS: u32 = 0x04;
        const ASSOCIATION_TRACK_VALUE: u32 = 0x40;
        const ASSOCIATION_CLIP_TIME: u32 = 0x4c;
        const ASSOCIATION_BLEND_RATE: u32 = 0x54;
        const ASSOCIATION_TRACK_COUNT: u32 = 0x64;
        const ASSOCIATION_TRACK_KEYS: [u32; 4] = [0x64, 0x68, 0x6c, 0x70];
        const ASSOCIATION_KEY_COUNT: u32 = 0x86;
        const MINUS_ONE: u32 = u32::MAX;
        const FIRE_ANIMATION_KEY: u32 = 0xc5;
        const FIRE_ANIMATION_KEY_ALT: u32 = 0xc6;
        const FIRE_KEY_SELECTOR: u32 = 0x0000_8000;
        const ALT_RATE_FLAG: u32 = 0x0000_8000;
        const RANDOMIZE_TIMING_FLAG: u32 = 0x0004_0000;
        const FORCE_PLAYBACK_FLAG: u32 = 0x0000_0008;
        const ADD_FINAL_FLAGS: u32 = 0x0000_0080;
        const CLEAR_ASSOCIATION_FLAG_A: u32 = 0x0000_0040;
        const CLEAR_ASSOCIATION_FLAG_B: u32 = 0x0000_0080;
        const SET_ASSOCIATION_FLAG_A: u32 = 0x0000_0010;
        const SET_ASSOCIATION_FLAG_B: u32 = 0x0000_8000;
        const SET_ASSOCIATION_FLAG_C: u32 = 0x0000_4000;
        const SET_ASSOCIATION_FLAG_D: u32 = 0x0020_0000;
        const GLOBAL_VERSION: u32 = 0x011d_6fd4;
        const VERSION_WITH_TIMING: u32 = 2;
        const SCALE_RANDOM: u32 = 0x00fe_8684;
        const RATE_RANDOM_SCALE: u32 = 0x00fe_87d8;
        const RATE_RANDOM_BIAS: u32 = 0x00fe_88bc;
        const U16_UNIT: u32 = 0x00fe_8680;
        const RATE_LOWER_BOUND: u32 = 0x00fe_870c;
        const RATE_UPPER_BOUND: u32 = 0x0105_7834;
        const RATE_VARIATION: u32 = 0x0105_7838;
        const RATE_REDUCTION: u32 = 0x00fe_8db0;
        const RANDOM_FACTOR: u32 = 0x00fe_8db0;
        const CALL_WEAPON_MANAGER: u32 = 1;
        const CALL_WEAPON_INFO: u32 = 2;
        const CALL_PED_LINK: u32 = 3;
        const CALL_FIND_ANIMATION: u32 = 4;
        const CALL_CREATE_ANIMATION: u32 = 5;
        const CALL_VERSION_GATE: u32 = 6;
        const CALL_PED_VIRTUAL: u32 = 7;
        const CALL_RANDOM: u32 = 8;
        const CALL_SAMPLE_TRACK: u32 = 9;
        const CALL_COUNT_TRACKS: u32 = 10;
        const CALL_SET_CALLBACK: u32 = 11;
        const CALL_SET_CLIP_TIME: u32 = 12;
        const CALL_UPDATE_BLEND: u32 = 13;
        const CALL_RANDOM_RANGE: u32 = 14;

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
        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }
        #[inline(always)]
        unsafe fn write_f32(address: u32, value: f32) {
            unsafe { write_u32(address, value.to_bits()) }
        }
        #[inline(always)]
        unsafe fn global_u32(file_va: u32) -> u32 {
            unsafe { read_u32(lf_checker_rt::global::<u32>(file_va) as u32) }
        }
        #[inline(always)]
        unsafe fn global_f32(file_va: u32) -> f32 {
            f32::from_bits(unsafe { global_u32(file_va) })
        }
        #[inline(always)]
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        fn sub_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn mul_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn cvttss2si(value: f32) -> i32 {
            let value = core::hint::black_box(value);
            // The original uses CVTTSS2SI: NaN, +infinity, values at or above
            // 2^31, and values below -2^31 produce the integer-indefinite
            // value. The cast below is reached only inside that finite range.
            if !value.is_finite() || value >= 2_147_483_648.0 || value < -2_147_483_648.0 {
                i32::MIN
            } else {
                value.trunc() as i32
            }
        }

        let weapon_manager = lf_checker_rt::callee_thiscall!(
            CALL_WEAPON_MANAGER,
            u32,
            ped.wrapping_add(PED_WEAPON_INFO),
        );
        let manager_key = read_u32(weapon_manager.wrapping_add(MANAGER_INFO_KEY));
        let weapon_info = lf_checker_rt::callee_cdecl!(CALL_WEAPON_INFO, u32, manager_key);
        let ped_has_link = lf_checker_rt::callee_thiscall!(CALL_PED_LINK, u32, ped) as u8 != 0;

        let (primary_key, secondary_key) = if ped_has_link {
            let requested_primary = read_u32(task.wrapping_add(TASK_LINK_FIRE));
            if requested_primary != MINUS_ONE {
                let requested_secondary = read_u32(task.wrapping_add(TASK_LINK_FIRE_SECONDARY));
                if requested_secondary != MINUS_ONE {
                    (requested_primary, requested_secondary)
                } else {
                    let info_primary = read_u32(weapon_info.wrapping_add(INFO_PRIMARY_KEY));
                    if info_primary == MINUS_ONE {
                        (info_primary, FIRE_ANIMATION_KEY)
                    } else if read_u32(weapon_info.wrapping_add(INFO_FLAGS)) & FIRE_KEY_SELECTOR != 0 {
                        (info_primary, FIRE_ANIMATION_KEY_ALT)
                    } else {
                        (info_primary, FIRE_ANIMATION_KEY)
                    }
                }
            } else {
                let info_primary = read_u32(weapon_info.wrapping_add(INFO_PRIMARY_KEY));
                if info_primary == MINUS_ONE {
                    (info_primary, FIRE_ANIMATION_KEY)
                } else if read_u32(weapon_info.wrapping_add(INFO_FLAGS)) & FIRE_KEY_SELECTOR != 0 {
                    (info_primary, FIRE_ANIMATION_KEY_ALT)
                } else {
                    (info_primary, FIRE_ANIMATION_KEY)
                }
            }
        } else {
            let requested_primary = read_u32(task.wrapping_add(TASK_LINK_PRIMARY));
            let requested_secondary = read_u32(task.wrapping_add(TASK_LINK_SECONDARY));
            if requested_primary != MINUS_ONE && requested_secondary != MINUS_ONE {
                (requested_primary, requested_secondary)
            } else {
                (read_u32(weapon_info.wrapping_add(INFO_PRIMARY_KEY)), FIRE_ANIMATION_KEY)
            }
        };

        let animation_manager = read_u32(ped.wrapping_add(PED_ANIMATION_MANAGER));
        let mut association = lf_checker_rt::callee_thiscall!(
            CALL_FIND_ANIMATION,
            u32,
            animation_manager,
            primary_key,
            secondary_key,
        );
        if association == 0 {
            association = lf_checker_rt::callee_thiscall!(
                CALL_CREATE_ANIMATION,
                u32,
                animation_manager,
                primary_key,
                secondary_key,
                read_u32(task.wrapping_add(TASK_INITIAL_BLEND)),
                MINUS_ONE,
            );
        }
        write_u32(task.wrapping_add(TASK_ASSOCIATION), association);

        let task_flags = read_u32(task.wrapping_add(TASK_FLAGS));
        let mut playback_rate = if task_flags & ALT_RATE_FLAG != 0 {
            read_f32(weapon_info.wrapping_add(INFO_ALTERNATE_RATE))
        } else {
            read_f32(weapon_info.wrapping_add(INFO_STANDARD_RATE))
        };

        if global_u32(GLOBAL_VERSION) == VERSION_WITH_TIMING
            && lf_checker_rt::callee_cdecl!(CALL_VERSION_GATE, u32,) as u8 != 0
            && lf_checker_rt::callee_thiscall!(CALL_PED_VIRTUAL, u32, ped) as u8 != 0
        {
            let scale_data = read_u32(ped.wrapping_add(PED_SCALE_DATA));
            if scale_data != 0 {
                playback_rate = mul_f32(
                    read_f32(scale_data.wrapping_add(SCALE_DATA_FIRE_RATE)),
                    playback_rate,
                );
            }
        }

        if lf_checker_rt::callee_thiscall!(CALL_PED_VIRTUAL, u32, ped) as u8 == 0 {
            let random = lf_checker_rt::callee_cdecl!(CALL_RANDOM, u32,) as i32 as f32;
            let random_rate = mul_f32(random, global_f32(SCALE_RANDOM));
            let random_rate = mul_f32(random_rate, global_f32(RATE_RANDOM_SCALE));
            let random_rate = add_f32(random_rate, global_f32(RATE_RANDOM_BIAS));
            write_f32(
                association.wrapping_add(ASSOCIATION_BLEND_RATE),
                mul_f32(random_rate, playback_rate),
            );
        } else {
            write_f32(
                association.wrapping_add(ASSOCIATION_BLEND_RATE),
                playback_rate,
            );
        }

        let sampled_track_value = read_u32(association.wrapping_add(ASSOCIATION_TRACK_VALUE));
        for (destination, source) in [
            (TASK_TRACK_KEYS, ASSOCIATION_TRACK_KEYS[0]),
            (TASK_TRACK_KEYS + 4, ASSOCIATION_TRACK_KEYS[1]),
            (TASK_TRACK_KEYS + 8, ASSOCIATION_TRACK_KEYS[2]),
            (TASK_TRACK_KEYS + 12, ASSOCIATION_TRACK_KEYS[3]),
        ] {
            let track_value = read_u32(association.wrapping_add(source));
            let _ = lf_checker_rt::callee_thiscall!(
                CALL_SAMPLE_TRACK,
                u32,
                association,
                sampled_track_value,
            );
            write_u32(task.wrapping_add(destination), track_value);
        }

        let track_count = lf_checker_rt::callee_thiscall!(
            CALL_COUNT_TRACKS,
            u32,
            association,
            0x200,
            0,
            1.0f32.to_bits(),
        );
        write_u32(task.wrapping_add(TASK_TRACK_COUNT), track_count);

        let animation_time = read_f32(association.wrapping_add(ASSOCIATION_CLIP_TIME));
        let first_track_time = f32::from_bits(read_u32(task.wrapping_add(TASK_TRACK_KEYS)));
        let playback_mode = read_u32(task.wrapping_add(TASK_PLAYBACK_MODE));
        if animation_time >= first_track_time && playback_mode == 1 {
            let playback_variant = read_u32(task.wrapping_add(TASK_PLAYBACK_VARIANT));
            let next_mode = if playback_variant == 3 {
                playback_variant
            } else if playback_variant == 5 {
                5
            } else {
                7
            };
            write_u32(task.wrapping_add(TASK_PLAYBACK_MODE), next_mode);
        }
        if task_flags & FORCE_PLAYBACK_FLAG != 0 {
            write_u32(task.wrapping_add(TASK_PLAYBACK_MODE), 3);
            write_u32(task.wrapping_add(TASK_RANDOM_VARIANT), 7);
        }

        let callback_address = lf_checker_rt::relocated(0x00dc_57c0);
        let _ = lf_checker_rt::callee_thiscall!(
            CALL_SET_CALLBACK,
            u32,
            association,
            2,
            callback_address,
            task,
        );
        let _ = lf_checker_rt::callee_thiscall!(
            CALL_SET_CLIP_TIME,
            u32,
            association,
            0.0f32.to_bits(),
        );
        let _ = lf_checker_rt::callee_thiscall!(
            CALL_UPDATE_BLEND,
            u32,
            association,
            animation_time.to_bits(),
        );

        let mut association_flags = read_u32(association.wrapping_add(ASSOCIATION_FLAGS));
        if association_flags & CLEAR_ASSOCIATION_FLAG_A != 0 {
            association_flags &= !CLEAR_ASSOCIATION_FLAG_A;
            write_u32(association.wrapping_add(ASSOCIATION_FLAGS), association_flags);
        }
        if association_flags & CLEAR_ASSOCIATION_FLAG_B != 0 {
            association_flags &= !CLEAR_ASSOCIATION_FLAG_B;
            write_u32(association.wrapping_add(ASSOCIATION_FLAGS), association_flags);
        }
        association_flags |= SET_ASSOCIATION_FLAG_A;
        write_u32(association.wrapping_add(ASSOCIATION_FLAGS), association_flags);
        association_flags |= SET_ASSOCIATION_FLAG_B;
        write_u32(association.wrapping_add(ASSOCIATION_FLAGS), association_flags);
        association_flags |= SET_ASSOCIATION_FLAG_C;
        write_u32(association.wrapping_add(ASSOCIATION_FLAGS), association_flags);

        if task_flags & RANDOMIZE_TIMING_FLAG != 0 {
            let minimum = read_u16(task.wrapping_add(TASK_RANDOM_MIN));
            let maximum = read_u16(task.wrapping_add(TASK_RANDOM_MAX));
            if (minimum as i16) > 0 && (maximum as i16) > 0 {
                let random = (lf_checker_rt::callee_cdecl!(CALL_RANDOM, u32,) & 0xffff) as f32;
                let minimum_signed = minimum as i16 as i32;
                let maximum_signed = maximum as i16 as i32;
                let span = maximum_signed.wrapping_sub(minimum_signed) as f32;
                let fraction = mul_f32(random, global_f32(U16_UNIT));
                let offset = cvttss2si(mul_f32(fraction, span));
                let selected = minimum_signed.wrapping_add(offset) as u16;
                write_u16(task.wrapping_add(TASK_SELECTED_KEY), selected);
            } else {
                let ped_weapon_key = read_u32(ped.wrapping_add(MANAGER_INFO_KEY));
                let fallback_info = lf_checker_rt::callee_cdecl!(
                    CALL_WEAPON_INFO,
                    u32,
                    ped_weapon_key,
                );
                let fallback_count = read_u16(fallback_info.wrapping_add(INFO_ANIMATION_COUNT));
                write_u16(task.wrapping_add(TASK_SELECTED_KEY), fallback_count);

                let pedal_rate_byte = read_u8(ped.wrapping_add(PED_ANIMATION_RATE_BYTE));
                let pedal_rate = mul_f32(
                    pedal_rate_byte as f32,
                    read_f32(ped.wrapping_add(PED_ANIMATION_RATE)),
                );
                let converted = cvttss2si(pedal_rate) as u16;
                let mut duration = if (converted as i16) < 100 {
                    converted
                } else {
                    100
                };

                if (fallback_count as i16) > 1 {
                    let duration_signed = (duration as i16) as i32;
                    let mut timing_limit = global_f32(RATE_UPPER_BOUND);
                    let duration_rate = mul_f32(
                        mul_f32(duration_signed as f32, global_f32(RATE_LOWER_BOUND)),
                        global_f32(RATE_VARIATION),
                    );
                    if duration_rate > timing_limit {
                        timing_limit = duration_rate;
                    }

                    let divisor = read_u32(weapon_info.wrapping_add(INFO_DIVISOR)) as i32;
                    let time_unit = if divisor == 0 { 30 } else { 1000 / divisor + 1 };
                    let base_duration = cvttss2si(mul_f32(time_unit as f32, timing_limit));

                    if global_u32(GLOBAL_VERSION) == VERSION_WITH_TIMING {
                        let random = (lf_checker_rt::callee_cdecl!(CALL_RANDOM, u32,) & 0xffff) as f32;
                        let random_fraction = mul_f32(random, global_f32(U16_UNIT));
                        let reduction = cvttss2si(mul_f32(
                            random_fraction,
                            global_f32(RANDOM_FACTOR),
                        ));
                        let adjusted = (base_duration
                            .wrapping_sub(reduction)
                            .wrapping_sub(1)) as u16;
                        duration = if (adjusted as i16) >= 1 { adjusted } else { 1 };
                    } else {
                        let random = (lf_checker_rt::callee_cdecl!(CALL_RANDOM, u32,) & 0xffff) as f32;
                        let random_fraction = mul_f32(random, global_f32(U16_UNIT));
                        let variation = cvttss2si(mul_f32(
                            random_fraction,
                            global_f32(RATE_REDUCTION),
                        ));
                        let adjustment = (0xffffu32.wrapping_sub(variation as u32) as u16) as i16 as i32;
                        let signed_base = (base_duration as u16) as i16 as i32;
                        if adjustment.wrapping_add(signed_base) < 1 {
                            duration = 1;
                        } else {
                            let jitter = lf_checker_rt::callee_cdecl!(
                                CALL_RANDOM_RANGE,
                                u32,
                                u32::MAX,
                                1,
                            ) as i32;
                            duration = (base_duration.wrapping_add(jitter)) as u16;
                        }
                    }
                    write_u16(task.wrapping_add(TASK_SELECTED_KEY), duration);
                }
            }
        }

        let selected_key = read_u16(task.wrapping_add(TASK_SELECTED_KEY));
        let animation_limit = read_u16(ped.wrapping_add(PED_ANIMATION_LIMIT));
        let bounded_key = if selected_key == u16::MAX {
            animation_limit
        } else if (selected_key as i16) < (animation_limit as i16) {
            selected_key
        } else {
            animation_limit
        };
        write_u16(task.wrapping_add(TASK_SELECTED_KEY), bounded_key);

        if task_flags & ADD_FINAL_FLAGS != 0 {
            association_flags = read_u32(association.wrapping_add(ASSOCIATION_FLAGS));
            association_flags |= 1 | SET_ASSOCIATION_FLAG_D;
            write_u32(association.wrapping_add(ASSOCIATION_FLAGS), association_flags);
        }
        0
    }
});
