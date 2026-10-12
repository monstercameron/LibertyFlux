// original: 0x00D020C0 CTaskComplexCombat::vf20

//! Advance the combat task for one tick and return the replacement subtask,
//! a created task, or null when a scripted decision consumes the request.
//!
//! The task owns the active subtask, target, timers, and combat flags; the
//! second argument is the ped being updated. The tick first marks the ped,
//! evaluates target and group conditions, ages active timers, and then
//! chooses between the idle gate, combat checks, threat responses, speech,
//! or a new subtask. Every game service call remains an explicit call through
//! the checker runtime so the proof can compare its receiver and arguments.
//!
//! One service also receives the incoming frame pointer as a second argument.
//! The portable Rust ABI does not expose that register; the contract pins it
//! to zero and compares the argument, other calls, heap writes, and result.
//!
//! Original: thiscall task tick with one stack argument and a 32-bit result.
lf_checker_rt::export!(thiscall, rw_00d020c0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_SUBTASK: u32 = 0x08;
        const TASK_FLAGS: u32 = 0x0c;
        const TASK_TARGET: u32 = 0x3c;
        const TASK_PATROL_TIMER: u32 = 0x58;
        const TASK_DELAY_TIMER: u32 = 0x5c;
        const TASK_STATE_FLAGS: u32 = 0x60;
        const TASK_LAST_SIGHTING: u32 = 0x6c;
        const TASK_COOLDOWN_A: u32 = 0x70;
        const TASK_COOLDOWN_B: u32 = 0x74;
        const TASK_RESPONSE_TIMER: u32 = 0x94;
        const SUBTASK_FLAGS: u32 = 0x0c;
        const SUBTASK_STATE: u32 = 0x18;
        const SUBTASK_KIND: u32 = 0x20;
        const PED_POSITION: u32 = 0x20;
        const PED_GROUP: u32 = 0x21c;
        const PED_GROUP_EXT: u32 = 0x224;
        const PED_FLAGS: u32 = 0x264;
        const PED_MODE_FLAGS: u32 = 0x26c;
        const PED_STATE: u32 = 0xa60;
        const PED_GROUP_LINK: u32 = 0xb30;
        const PED_TASK_FLAGS: u32 = 0x29c;
        const PED_ACTIVITY_FLAGS: u32 = 0x2a0;
        const PED_GROUP_VALUE: u32 = 0xd68;
        const PED_SPEECH: u32 = 0x570;
        const TARGET_KIND_FLAGS: u32 = 0x218;
        const TARGET_STATE: u32 = 0x219;
        const TARGET_GROUP: u32 = 0x224;
        const TARGET_GROUP_MEMBER: u32 = 0x228;
        const TARGET_FLAGS: u32 = 0x264;
        const TARGET_STATE_FLAGS: u32 = 0xa60;
        const TARGET_BUSY: u32 = 0xa70;
        const TARGET_GROUP_LINK: u32 = 0xb30;
        const GROUP_SIZE: u32 = 0x12c;
        const GROUP_FLAGS: u32 = 0x38;
        const GROUP_VALUE: u32 = 0x50;
        const GROUP_ALERT: u32 = 0x2c8;
        const TARGET_GROUP_LIST: u32 = 0x2e0;
        const NODE_STATE_BYTE: u32 = 0xcb;
        const TARGET_MEMORY_FLAG: u32 = 0xf1f;
        const POSITION_X: u32 = 0x30;
        const POSITION_Y: u32 = 0x34;
        const POSITION_Z: u32 = 0x38;
        const FRAME_ZERO: f32 = 0.0;
        const FIVE_SECONDS_BITS: u32 = 0x40a0_0000;
        const ONE_AND_HALF_BITS: u32 = 0x3fc0_0000;
        const TWO_SECONDS_BITS: u32 = 0x4000_0000;
        const ONE_SECOND_BITS: u32 = 0x3f80_0000;
        const TIMER_RESTART_BITS: u32 = 0x38d1_b717;
        const TYPE_ARMED: u32 = 0x76d;
        const TYPE_DRAWING: u32 = 0x76f;
        const TYPE_TARGETED: u32 = 0x771;
        const TYPE_THREATENED: u32 = 0x772;
        const TYPE_IDLE: u32 = 0x770;
        const TYPE_RUNNING: u32 = 0x776;
        const TYPE_SEARCH: u32 = 0x779;
        const TYPE_GANG: u32 = 0x13f;
        const TYPE_HELP: u32 = 0x2e6;
        const TYPE_SEARCH_ALT: u32 = 0x2e7;
        const TYPE_SPECIAL: u32 = 0x454;
        const GROUP_KIND: u32 = 2;
        const TASK_FIND_END: u32 = 0;
        const SPEECH_DRAW_GUN: u32 = 0x00ed_fb80;

        const G_FRAME_DELTA: u32 = 0x0117_35bc;
        const G_RANGE_LIMIT: u32 = 0x0105_3e74;
        const G_ALERT_LIMIT: u32 = 0x00e9_88a0;
        const G_GROUP_ALERT: u32 = 0x00e9_d786;
        const G_RANDOM_SCALE: u32 = 0x00fe_8684;
        const G_RANDOM_SMALL: u32 = 0x00fe_879c;
        const G_RANDOM_MEDIUM: u32 = 0x00fe_87d8;
        const G_RANDOM_BIAS: u32 = 0x00fe_88bc;
        const G_RANDOM_LARGE: u32 = 0x00fe_8a94;
        const G_RANDOM_BASE: u32 = 0x00fe_8ad8;
        const G_SHORT_RANGE: u32 = 0x00fe_8b40;
        const G_TASK_POOL: u32 = 0x0167_e2a0;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            unsafe { f32::from_bits(read_u32(address)) }
        }
        #[inline(always)]
        unsafe fn write_f32(address: u32, value: f32) {
            unsafe { write_u32(address, value.to_bits()) }
        }
        #[inline(always)]
        unsafe fn global_f32(file_address: u32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::global::<u32>(file_address).read_unaligned()) }
        }
        #[inline(always)]
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(
                core::hint::black_box(left) + core::hint::black_box(right),
            )
        }
        #[inline(always)]
        fn sub_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(
                core::hint::black_box(left) - core::hint::black_box(right),
            )
        }
        #[inline(always)]
        fn mul_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(
                core::hint::black_box(left) * core::hint::black_box(right),
            )
        }
        #[inline(always)]
        unsafe fn subtask_type(_call_id: u32, task: u32) -> u32 {
            let subtask = unsafe { read_u32(task.wrapping_add(TASK_SUBTASK)) };
            // Every queried subtask uses one virtual table slot, so the
            // checker records those calls through the slot's single stub.
            lf_checker_rt::callee_thiscall!(CALL_002, u32, subtask)
        }
        #[inline(always)]
        unsafe fn object_type(call_id: u32, object: u32) -> u32 {
            lf_checker_rt::callee_thiscall!(call_id, u32, object)
        }
        #[inline(always)]
        unsafe fn ped_predicate(call_id: u32, receiver: u32, ped: u32) -> bool {
            (lf_checker_rt::callee_thiscall!(call_id, u32, receiver, ped, 1, 0) & 0xff) != 0
        }
        #[inline(always)]
        unsafe fn task_find(call_id: u32, list: u32, kind: u32) -> bool {
            (lf_checker_rt::callee_thiscall!(call_id, u32, list, kind, TASK_FIND_END) & 0xff) != 0
        }
        #[inline(always)]
        unsafe fn slot_128(call_id: u32, object: u32) -> bool {
            (lf_checker_rt::callee_thiscall!(call_id, u32, object) & 0xff) != 0
        }
        #[inline(always)]
        unsafe fn slot_zero(call_id: u32, object: u32, argument: u32) -> u32 {
            lf_checker_rt::callee_thiscall!(call_id, u32, object, argument)
        }
        #[inline(always)]
        unsafe fn vector_distance_squared(left: u32, right: u32) -> f32 {
            let delta_y = sub_f32(
                unsafe { read_f32(left.wrapping_add(POSITION_Y)) },
                unsafe { read_f32(right.wrapping_add(POSITION_Y)) },
            );
            let delta_x = sub_f32(
                unsafe { read_f32(left.wrapping_add(POSITION_X)) },
                unsafe { read_f32(right.wrapping_add(POSITION_X)) },
            );
            let delta_z = sub_f32(
                unsafe { read_f32(left.wrapping_add(POSITION_Z)) },
                unsafe { read_f32(right.wrapping_add(POSITION_Z)) },
            );
            let y_squared = mul_f32(delta_y, delta_y);
            let x_squared = mul_f32(delta_x, delta_x);
            let z_squared = mul_f32(delta_z, delta_z);
            add_f32(add_f32(y_squared, x_squared), z_squared)
        }

        const CALL_001: u32 = 1;
        const CALL_002: u32 = 2;
        const CALL_003: u32 = 3;
        const CALL_004: u32 = 4;
        const CALL_005: u32 = 5;
        const CALL_006: u32 = 6;
        const CALL_007: u32 = 7;
        const CALL_008: u32 = 8;
        const CALL_009: u32 = 9;
        const CALL_010: u32 = 10;
        const CALL_011: u32 = 11;
        const CALL_012: u32 = 12;
        const CALL_013: u32 = 13;
        const CALL_014: u32 = 14;
        const CALL_015: u32 = 15;
        const CALL_016: u32 = 16;
        const CALL_017: u32 = 17;
        const CALL_018: u32 = 18;
        const CALL_019: u32 = 19;
        const CALL_020: u32 = 20;
        const CALL_021: u32 = 21;
        const CALL_022: u32 = 22;
        const CALL_023: u32 = 23;
        const CALL_024: u32 = 24;
        const CALL_025: u32 = 25;
        const CALL_026: u32 = 26;
        const CALL_027: u32 = 27;
        const CALL_028: u32 = 28;
        const CALL_029: u32 = 29;
        const CALL_030: u32 = 30;
        const CALL_031: u32 = 31;
        const CALL_032: u32 = 32;
        const CALL_033: u32 = 33;
        const CALL_034: u32 = 34;
        const CALL_035: u32 = 35;
        const CALL_036: u32 = 36;
        const CALL_037: u32 = 37;
        const CALL_038: u32 = 38;
        const CALL_039: u32 = 39;
        const CALL_041: u32 = 41;
        const CALL_042: u32 = 42;
        const CALL_043: u32 = 43;
        const CALL_044: u32 = 44;
        const CALL_045: u32 = 45;
        const CALL_046: u32 = 46;
        const CALL_047: u32 = 47;
        const CALL_048: u32 = 48;
        const CALL_049: u32 = 49;
        const CALL_050: u32 = 50;
        const CALL_051: u32 = 51;
        const CALL_052: u32 = 52;
        const CALL_053: u32 = 53;
        const CALL_054: u32 = 54;
        const CALL_055: u32 = 55;
        const CALL_056: u32 = 56;
        const CALL_057: u32 = 57;
        const CALL_058: u32 = 58;
        const CALL_060: u32 = 60;
        const CALL_061: u32 = 61;
        const CALL_062: u32 = 62;
        const CALL_063: u32 = 63;
        const CALL_064: u32 = 64;
        const CALL_065: u32 = 65;
        const CALL_066: u32 = 66;
        const CALL_067: u32 = 67;
        const CALL_069: u32 = 69;
        const CALL_070: u32 = 70;
        const CALL_071: u32 = 71;
        const CALL_072: u32 = 72;
        const CALL_073: u32 = 73;
        const CALL_074: u32 = 74;
        const CALL_075: u32 = 75;
        const CALL_076: u32 = 76;
        const CALL_077: u32 = 77;
        const CALL_078: u32 = 78;
        const CALL_079: u32 = 79;
        const CALL_080: u32 = 80;
        const CALL_081: u32 = 81;
        const CALL_082: u32 = 82;
        const CALL_083: u32 = 83;
        const CALL_084: u32 = 84;
        const CALL_085: u32 = 85;
        const CALL_086: u32 = 86;
        const CALL_087: u32 = 87;
        const CALL_088: u32 = 88;
        const CALL_089: u32 = 89;
        const CALL_090: u32 = 90;
        const CALL_091: u32 = 91;
        const CALL_092: u32 = 92;
        const CALL_093: u32 = 93;
        const CALL_094: u32 = 94;
        const CALL_095: u32 = 95;
        const CALL_096: u32 = 96;
        const CALL_097: u32 = 97;
        const CALL_098: u32 = 98;
        const CALL_099: u32 = 99;
        const CALL_100: u32 = 100;
        const CALL_101: u32 = 101;
        const CALL_102: u32 = 102;
        const CALL_103: u32 = 103;
        const CALL_104: u32 = 104;
        const CALL_105: u32 = 105;
        const CALL_106: u32 = 106;
        const CALL_107: u32 = 107;
        const CALL_108: u32 = 108;
        const CALL_109: u32 = 109;
        const CALL_110: u32 = 110;
        const CALL_111: u32 = 111;
        const CALL_112: u32 = 112;
        const CALL_113: u32 = 113;
        const CALL_114: u32 = 114;
        const CALL_115: u32 = 115;
        const CALL_116: u32 = 116;
        const CALL_117: u32 = 117;
        const CALL_118: u32 = 118;
        const CALL_119: u32 = 119;
        const CALL_120: u32 = 120;

        write_u32(
            ped.wrapping_add(PED_ACTIVITY_FLAGS),
            read_u32(ped.wrapping_add(PED_ACTIVITY_FLAGS)) | 0x40,
        );

        let local_player = lf_checker_rt::callee_cdecl!(CALL_001, u32,);
        if local_player != 0 && subtask_type(CALL_002, this) != TYPE_IDLE {
            let subtask = read_u32(this.wrapping_add(TASK_SUBTASK));
            let target = read_u32(this.wrapping_add(TASK_TARGET));
            let mut target_is_engaged = false;
            if target != 0
                && read_u8(target.wrapping_add(TARGET_KIND_FLAGS)) == 0
                && read_u8(target.wrapping_add(TARGET_STATE)) != 0
            {
                if read_u32(target.wrapping_add(TARGET_BUSY)) == 1 {
                    target_is_engaged = true;
                } else {
                    let target_group = read_u32(target.wrapping_add(TARGET_GROUP));
                    if lf_checker_rt::callee_thiscall!(CALL_003, u32, target_group) != 0 {
                        let candidate =
                            lf_checker_rt::callee_thiscall!(CALL_004, u32, target_group);
                        target_is_engaged = object_type(CALL_005, candidate) == TYPE_SPECIAL;
                    }
                }
            }

            let ped_state = read_u8(ped.wrapping_add(PED_STATE));
            let ped_mask = read_u32(ped.wrapping_add(PED_FLAGS));
            let special_task_mode = !(ped_state == 2 && ped_mask & 0x1000_0000 == 0);

            let mode_flags = read_u8(ped.wrapping_add(PED_MODE_FLAGS));
            let mode_target = read_u32(ped.wrapping_add(PED_GROUP_LINK));
            let has_special_mode = if mode_flags & 4 != 0 && mode_target != 0 {
                (lf_checker_rt::callee_thiscall!(CALL_006, u32, mode_target, ped) & 0xff) != 0
                    && read_u32(mode_target.wrapping_add(0x1304)) == 4
            } else {
                false
            };

            let local_context = lf_checker_rt::callee_cdecl!(CALL_007, u32,);
            let has_combat_state = (read_u8(local_context.wrapping_add(0x5a)) & 4 != 0)
                && special_task_mode;
            let has_group_state = if has_combat_state {
                false
            } else {
                let local_context_again = lf_checker_rt::callee_cdecl!(CALL_008, u32,);
                (read_u8(local_context_again.wrapping_add(0x5a)) & 7 != 0)
                    && read_u32(
                        read_u32(ped.wrapping_add(PED_GROUP)).wrapping_add(GROUP_SIZE),
                    ) == GROUP_KIND
            };
            let has_combat_state = has_combat_state || has_group_state;

            let target_response = if target != 0 {
                (lf_checker_rt::callee_thiscall!(
                    CALL_009,
                    u32,
                    target
                ) & 0xff)
                    != 0
                    && read_u8(target.wrapping_add(0x211)) != 0
            } else {
                false
            };
            let cutscene_active = if has_combat_state {
                false
            } else {
                (lf_checker_rt::callee_cdecl!(CALL_010, u32,) & 0xff) != 0
            };

            if !has_special_mode
                && (target_is_engaged || target_response || has_combat_state || cutscene_active)
            {
                let kind = subtask_type(CALL_011, this);
                let should_start_reaction = kind != TYPE_HELP
                    && (ped_predicate(CALL_013, subtask, ped)
                        || (has_combat_state && ped_predicate(CALL_014, subtask, ped)));
                if should_start_reaction {
                    let ped_group = read_u32(ped.wrapping_add(PED_GROUP));
                    if ped_state == 1
                        && read_u32(ped_group.wrapping_add(GROUP_SIZE)) != GROUP_KIND
                    {
                        return 0;
                    }
                    let manager = lf_checker_rt::global::<u32>(G_TASK_POOL).read_unaligned();
                    let allocated = lf_checker_rt::callee_thiscall!(CALL_015, u32, manager);
                    if allocated == 0 {
                        return 0;
                    }
                    return lf_checker_rt::callee_thiscall!(CALL_016, u32, allocated, target);
                }
                return subtask;
            }

            if subtask_type(CALL_012, this) == TYPE_HELP {
                let subtask = read_u32(this.wrapping_add(TASK_SUBTASK));
                write_u8(subtask.wrapping_add(SUBTASK_STATE), 1);
                return subtask;
            }
        }

        for timer_offset in [TASK_COOLDOWN_A, TASK_COOLDOWN_B] {
            let timer = read_f32(this.wrapping_add(timer_offset));
            if timer > FRAME_ZERO {
                write_f32(
                    this.wrapping_add(timer_offset),
                    sub_f32(timer, global_f32(G_FRAME_DELTA)),
                );
            }
        }

        let subtask = read_u32(this.wrapping_add(TASK_SUBTASK));

        let ped_state = read_u8(ped.wrapping_add(PED_STATE));
        let ped_group = read_u32(ped.wrapping_add(PED_GROUP));
        let ped_group_ext = read_u32(ped.wrapping_add(PED_GROUP_EXT));
        let target = read_u32(this.wrapping_add(TASK_TARGET));
        let needs_distance_check = ped_state == 1
            && read_u32(ped_group.wrapping_add(GROUP_SIZE)) != GROUP_KIND
            && {
                let kind_for_distance = subtask_type(CALL_017, this);
                kind_for_distance == TYPE_TARGETED
                    || (kind_for_distance != TYPE_TARGETED
                        && subtask_type(CALL_018, this) == 0x773)
            }
            && target != 0
            && (lf_checker_rt::callee_thiscall!(CALL_019, u32, ped_group_ext, target, 1) & 0xff)
                == 0;
        if needs_distance_check {
            let ped_position = read_u32(ped.wrapping_add(PED_POSITION));
            let target_position = read_u32(target.wrapping_add(PED_POSITION));
            let distance_squared = vector_distance_squared(ped_position, target_position);
            let range_limit = global_f32(G_RANGE_LIMIT);
            let range_squared = mul_f32(range_limit, range_limit);

            let mut active_target_range = false;
            if subtask_type(CALL_020, this) == TYPE_TARGETED {
                let list = read_u32(target.wrapping_add(TARGET_GROUP))
                    .wrapping_add(TARGET_GROUP_LIST);
                if task_find(CALL_021, list, TYPE_IDLE) {
                    let measured: f32 = lf_checker_rt::callee_thiscall!(
                        CALL_022,
                        f32,
                        target
                    );
                    active_target_range = global_f32(G_ALERT_LIMIT) > measured;
                }
            }

            if distance_squared > range_squared || active_target_range {
                let task_flags = read_u32(this.wrapping_add(TASK_STATE_FLAGS));
                if task_flags & 0x8000 != 0 && ped_predicate(CALL_023, subtask, ped) {
                    return 0;
                }
            } else {
                write_u32(
                    this.wrapping_add(TASK_STATE_FLAGS),
                    read_u32(this.wrapping_add(TASK_STATE_FLAGS)) | 0x8000,
                );
            }
        }

        let nearby_group = lf_checker_rt::callee_thiscall!(CALL_024, u32, ped);
        if nearby_group != 0 {
            let other_ped = lf_checker_rt::callee_thiscall!(
                CALL_025,
                u32,
                nearby_group.wrapping_add(8)
            );
            if other_ped != 0 && other_ped != ped {
                if read_u8(other_ped.wrapping_add(TARGET_STATE)) == 0 {
                    let blocked_by_subject = read_u8(ped.wrapping_add(PED_MODE_FLAGS)) & 4 != 0
                        && read_u32(ped.wrapping_add(PED_GROUP_LINK)) != 0;
                    let blocked_by_list = if blocked_by_subject {
                        false
                    } else {
                        let linked_group = read_u32(other_ped.wrapping_add(TARGET_GROUP));
                        let task_list = linked_group.wrapping_add(TARGET_GROUP_LIST);
                        task_find(CALL_029, task_list, 0x76c)
                            && !task_find(CALL_030, task_list, TYPE_THREATENED)
                            && (!task_find(CALL_031, task_list, TYPE_SEARCH)
                                || !task_find(CALL_032, task_list, 0x11d))
                    };
                    if blocked_by_subject || blocked_by_list {
                        // The native path joins the common ped update below.
                    } else {
                        let group_value = read_u32(
                            read_u32(ped.wrapping_add(PED_GROUP_EXT)).wrapping_add(GROUP_VALUE),
                        );
                        if group_value == 0 && ped_predicate(CALL_033, subtask, ped) {
                            return 0;
                        }
                    }
                } else {
                    let target_group = if target == 0 {
                        0
                    } else {
                        read_u32(target.wrapping_add(TARGET_GROUP))
                    };
                    let target_group_size = if target == 0 {
                        0
                    } else {
                        read_u32(target_group.wrapping_add(GROUP_SIZE))
                    };
                    let target_is_incompatible = target == 0
                        || target_group_size != GROUP_KIND
                        || (read_u8(target.wrapping_add(TARGET_STATE_FLAGS)) == 2
                            && read_u32(target.wrapping_add(TARGET_FLAGS)) & 0x0040_0000 == 0)
                        || {
                            let group_member = read_u32(
                                other_ped.wrapping_add(TARGET_GROUP_MEMBER),
                            );
                            group_member == 0
                                || group_member == 0xffff_ff90
                                || ((read_u8(group_member.wrapping_add(NODE_STATE_BYTE)) & 1 == 0)
                                    && (lf_checker_rt::callee_cdecl!(CALL_026, u32,) & 0xff) != 0)
                        };
                    let mut bypass_group_predicate = false;
                    if target_is_incompatible {
                        let blocked_by_subject = read_u8(ped.wrapping_add(PED_MODE_FLAGS)) & 4 != 0
                            && read_u32(ped.wrapping_add(PED_GROUP_LINK)) != 0;
                        let blocked_by_group = if blocked_by_subject {
                            false
                        } else {
                            let group = read_u32(ped.wrapping_add(PED_GROUP_EXT));
                            (read_u8(group.wrapping_add(GROUP_FLAGS)) & 1 == 0)
                                || (lf_checker_rt::callee_thiscall!(
                                    CALL_027,
                                    u32,
                                    group.wrapping_add(0x10),
                                    read_u32(ped.wrapping_add(PED_POSITION)).wrapping_add(POSITION_X),
                                    0
                                ) & 0xff)
                                    != 0
                                || (target != 0
                                    && (lf_checker_rt::callee_thiscall!(
                                        CALL_028,
                                        u32,
                                        group.wrapping_add(0x10),
                                        read_u32(target.wrapping_add(PED_POSITION))
                                            .wrapping_add(POSITION_X),
                                        0
                                    ) & 0xff)
                                        != 0)
                        };
                        bypass_group_predicate = blocked_by_subject || blocked_by_group;
                    }
                    if !bypass_group_predicate {
                        let group_value = read_u32(
                            read_u32(ped.wrapping_add(PED_GROUP_EXT)).wrapping_add(GROUP_VALUE),
                        );
                        if group_value == 0 && ped_predicate(CALL_033, subtask, ped) {
                            return 0;
                        }
                    }
                }
            }
        }

        let ped_slot_128 = slot_128(CALL_034, ped);
        if !ped_slot_128 && target != 0 && slot_128(CALL_035, target) {
            if subtask_type(CALL_036, this) != TYPE_IDLE {
                write_u32(
                    ped.wrapping_add(PED_TASK_FLAGS),
                    read_u32(ped.wrapping_add(PED_TASK_FLAGS)) | 0x8000_0000,
                );
            }
        }

        let random_for_response = lf_checker_rt::callee_cdecl!(CALL_037, u32,) as i32 as f32;
        let normalized_response = mul_f32(random_for_response, global_f32(G_RANDOM_SCALE));
        if global_f32(G_RANDOM_SMALL) > normalized_response {
            let _: u32 = lf_checker_rt::callee_thiscall!(CALL_038, u32, this, ped);
        }
        write_u32(
            ped.wrapping_add(PED_TASK_FLAGS),
            read_u32(ped.wrapping_add(PED_TASK_FLAGS)) | 0x4000_0000,
        );

        let mut needs_default_task = false;
        if subtask_type(CALL_039, this) != TYPE_RUNNING
            && target != 0
            && !slot_128(CALL_034, target)
        {
            let list = read_u32(target.wrapping_add(TARGET_GROUP))
                .wrapping_add(TARGET_GROUP_LIST);
            needs_default_task = task_find(CALL_041, list, TYPE_SEARCH_ALT);
        }

        if target == 0 || needs_default_task {
            if read_u8(subtask.wrapping_add(SUBTASK_FLAGS)) & 1 != 0 {
                return 0;
            }
            if (lf_checker_rt::callee_thiscall!(CALL_120, u32, subtask, ped, 0, 0) & 0xff) != 0 {
                write_u32(
                    subtask.wrapping_add(SUBTASK_FLAGS),
                    read_u32(subtask.wrapping_add(SUBTASK_FLAGS)) | 2,
                );
                return 0;
            }
            return subtask;
        }

        // The native service receives the caller's incoming frame value as
        // its second stack argument; this contract pins that entry register
        // to zero because the portable Rust ABI does not expose it.
        let _: u32 = lf_checker_rt::callee_thiscall!(CALL_042, u32, this, ped, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(CALL_043, u32, this, ped);
        let decision_group = read_u32(ped.wrapping_add(PED_GROUP_EXT));
        let group_decision = lf_checker_rt::callee_thiscall!(CALL_044, u32, decision_group, 1);

        if read_u8(ped.wrapping_add(PED_MODE_FLAGS)) & 4 == 0
            && subtask_type(CALL_045, this) != TYPE_IDLE
        {
            write_u32(
                ped.wrapping_add(PED_TASK_FLAGS),
                read_u32(ped.wrapping_add(PED_TASK_FLAGS)) | 0x1000_0000,
            );
        }
        let should_select_group_action = subtask_type(CALL_046, this) == TYPE_GANG
            || (subtask_type(CALL_047, this) == TYPE_ARMED
                && lf_checker_rt::callee_thiscall!(CALL_048, u32, group_decision, target) != 2);
        if should_select_group_action {
            let _: u32 = lf_checker_rt::callee_thiscall!(CALL_049, u32, ped, 1);
            if subtask_type(CALL_050, this) != TYPE_ARMED {
                let target_position = read_u32(target.wrapping_add(PED_POSITION));
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    CALL_051,
                    u32,
                    ped,
                    target_position.wrapping_add(POSITION_X)
                );
            }
        }

        let mut timer = read_f32(this.wrapping_add(TASK_DELAY_TIMER));
        if timer > FRAME_ZERO {
            timer = sub_f32(timer, global_f32(G_FRAME_DELTA));
            write_f32(this.wrapping_add(TASK_DELAY_TIMER), timer);
            if timer <= FRAME_ZERO && ped_predicate(CALL_052, subtask, ped) {
                return 0;
            }
            if timer <= FRAME_ZERO {
                write_u32(this.wrapping_add(TASK_DELAY_TIMER), TIMER_RESTART_BITS);
            }
        }

        timer = read_f32(this.wrapping_add(TASK_RESPONSE_TIMER));
        if timer > FRAME_ZERO {
            timer = sub_f32(timer, global_f32(G_FRAME_DELTA));
            write_f32(this.wrapping_add(TASK_RESPONSE_TIMER), timer);
            if timer <= FRAME_ZERO && ped_predicate(CALL_053, subtask, ped) {
                return 0;
            }
            if timer <= FRAME_ZERO {
                write_u32(this.wrapping_add(TASK_RESPONSE_TIMER), TIMER_RESTART_BITS);
            }
        }

        if (ped_state != 2 || read_u32(ped.wrapping_add(PED_FLAGS)) & 0x0040_0000 != 0)
            && read_u32(ped_group.wrapping_add(GROUP_SIZE)) == GROUP_KIND
        {
            let mut relevant_ped = lf_checker_rt::callee_thiscall!(CALL_054, u32, ped);
            if relevant_ped == 0 {
                relevant_ped = target;
            } else {
                let group_ped = lf_checker_rt::callee_thiscall!(CALL_055, u32, ped);
                relevant_ped = lf_checker_rt::callee_thiscall!(
                    CALL_056,
                    u32,
                    group_ped.wrapping_add(8)
                );
            }
            if relevant_ped != 0
                && read_u8(relevant_ped.wrapping_add(TARGET_STATE)) != 0
                && (lf_checker_rt::callee_cdecl!(CALL_057, u32,) & 0xff) == 0
                && ped_predicate(CALL_058, subtask, ped)
            {
                return 0;
            }
        }

        if target != 0
            && !slot_128(CALL_034, target)
            && ped_state == 1
        {
            let group_size = read_u32(ped_group.wrapping_add(GROUP_SIZE));
            let response_allowed = if group_size == GROUP_KIND {
                if read_u32(target.wrapping_add(TARGET_BUSY)) != 1 {
                    false
                } else {
                    let first = lf_checker_rt::callee_thiscall!(CALL_060, u32, target);
                    if first == 0 {
                        false
                    } else {
                        let second = lf_checker_rt::callee_thiscall!(CALL_061, u32, target);
                        if read_u8(second.wrapping_add(TARGET_MEMORY_FLAG)) & 0x40 == 0 {
                            false
                        } else {
                            let third = lf_checker_rt::callee_thiscall!(CALL_062, u32, target, target);
                            (lf_checker_rt::callee_thiscall!(CALL_063, u32, third) & 0xff) != 0
                                && subtask_type(CALL_064, this) != TYPE_RUNNING
                        }
                    }
                }
            } else {
                let list = read_u32(target.wrapping_add(TARGET_GROUP))
                    .wrapping_add(TARGET_GROUP_LIST);
                task_find(CALL_065, list, TYPE_SEARCH_ALT)
            };
            if response_allowed && ped_predicate(CALL_066, this, ped) {
                return 0;
            }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(CALL_067, u32, this, ped);

        if target == 0 {
            if read_u8(subtask.wrapping_add(SUBTASK_FLAGS)) & 1 != 0 {
                return 0;
            }
            if (lf_checker_rt::callee_thiscall!(CALL_120, u32, subtask, ped, 0, 0) & 0xff) != 0 {
                write_u32(
                    subtask.wrapping_add(SUBTASK_FLAGS),
                    read_u32(subtask.wrapping_add(SUBTASK_FLAGS)) | 2,
                );
                return 0;
            }
            return subtask;
        }

        let should_finish_subtask = (lf_checker_rt::callee_thiscall!(CALL_069, u32, this, ped)
            & 0xff)
            != 0
            && ped_predicate(CALL_070, subtask, ped)
            || (read_u32(this.wrapping_add(TASK_STATE_FLAGS)) & 8 == 0
                && ((lf_checker_rt::callee_thiscall!(CALL_071, u32, this, ped) & 0xff) != 0
                    || subtask_type(CALL_072, this) == TYPE_IDLE)
                && ped_predicate(CALL_073, subtask, ped))
            || (subtask_type(CALL_074, this) == TYPE_TARGETED
                && (lf_checker_rt::callee_thiscall!(
                    CALL_075,
                    u32,
                    target.wrapping_add(0x2b0),
                    1
                ) & 0xff)
                    != 0
                && ped_predicate(CALL_076, subtask, ped));
        if should_finish_subtask {
            return lf_checker_rt::callee_thiscall!(CALL_116, u32, this, ped, 0, 0);
        }

        if subtask_type(CALL_077, this) == TYPE_ARMED {
            let current = read_f32(this.wrapping_add(TASK_LAST_SIGHTING));
            write_f32(
                this.wrapping_add(TASK_LAST_SIGHTING),
                add_f32(global_f32(G_FRAME_DELTA), current),
            );
        } else if read_u32(this.wrapping_add(TASK_STATE_FLAGS)) & 0x1000 == 0 {
            write_u32(this.wrapping_add(TASK_LAST_SIGHTING), 0);
        }

        if read_u32(ped.wrapping_add(0x260)) & 0x0080_0000 == 0
            && subtask_type(CALL_078, this) == TYPE_ARMED
            && read_u32(ped.wrapping_add(PED_GROUP_VALUE)) == 0
        {
            let world_state = lf_checker_rt::callee_thiscall!(
                CALL_079,
                u32,
                ped_group_ext
            );
            if (read_u8(world_state.wrapping_add(0x8f4)) & 0x60) == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(CALL_080, u32, ped, 1, 1000);
            }
        } else if read_u8(ped_group_ext.wrapping_add(GROUP_ALERT))
            < read_u8(lf_checker_rt::global::<u8>(G_GROUP_ALERT) as u32)
        {
            if read_u32(ped.wrapping_add(0x260)) & 0x0080_0000 == 0
                && read_u32(ped.wrapping_add(PED_GROUP_VALUE)) == 0
                && read_u32(this.wrapping_add(TASK_STATE_FLAGS)) & 0x600 != 0
            {
                let _: u32 = lf_checker_rt::callee_thiscall!(CALL_080, u32, ped, 1, 1000);
            }
        } else {
            write_u32(this.wrapping_add(TASK_LAST_SIGHTING), 0);
        }

        if read_u32(this.wrapping_add(TASK_STATE_FLAGS)) & 0x400 != 0
            && read_u32(this.wrapping_add(TASK_FLAGS)) & 2 != 0
            && ped_predicate(CALL_081, subtask, ped)
        {
            write_u32(this.wrapping_add(TASK_PATROL_TIMER), FIVE_SECONDS_BITS);
            let random = lf_checker_rt::callee_cdecl!(CALL_082, u32,) as i32 as f32;
            let factor = add_f32(
                mul_f32(
                    mul_f32(random, global_f32(G_RANDOM_SCALE)),
                    global_f32(G_RANDOM_MEDIUM),
                ),
                global_f32(G_RANDOM_BIAS),
            );
            let new_timer = mul_f32(
                factor,
                read_f32(this.wrapping_add(TASK_PATROL_TIMER)),
            );
            write_f32(this.wrapping_add(TASK_PATROL_TIMER), new_timer);
            return lf_checker_rt::callee_thiscall!(CALL_083, u32, this, TYPE_ARMED, ped);
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(CALL_084, u32, this, ped);

        let armed_kind = subtask_type(CALL_085, this);
        let distance_kind = armed_kind == TYPE_ARMED
            && read_f32(this.wrapping_add(TASK_COOLDOWN_B)) <= FRAME_ZERO;
        let is_targeted = if distance_kind {
            true
        } else {
            subtask_type(CALL_086, this) == TYPE_TARGETED
        };
        if is_targeted {
            let timer = read_f32(this.wrapping_add(TASK_PATROL_TIMER));
            if timer > FRAME_ZERO {
                let after_tick = sub_f32(timer, global_f32(G_FRAME_DELTA));
                write_f32(this.wrapping_add(TASK_PATROL_TIMER), after_tick);
                if after_tick <= FRAME_ZERO {
                    write_u32(
                        this.wrapping_add(TASK_STATE_FLAGS),
                        read_u32(this.wrapping_add(TASK_STATE_FLAGS)) & !0x400,
                    );
                    let response = lf_checker_rt::callee_thiscall!(
                        CALL_087,
                        u32,
                        this,
                        ped,
                        0,
                        1
                    );
                    if response != 0 {
                        if ped_predicate(CALL_088, subtask, ped) {
                            return response;
                        }
                        let _: u32 = slot_zero(CALL_089, response, 1);
                    }

                    if subtask_type(CALL_090, this) == TYPE_TARGETED {
                        write_u32(this.wrapping_add(TASK_PATROL_TIMER), FIVE_SECONDS_BITS);
                    } else if lf_checker_rt::callee_cdecl!(CALL_091, u32, ped, target) == 0x78b {
                        write_u32(this.wrapping_add(TASK_PATROL_TIMER), ONE_AND_HALF_BITS);
                    } else if read_u32(ped.wrapping_add(PED_GROUP_VALUE)) == 0 {
                        let no_target_pressure = read_u32(target.wrapping_add(TARGET_GROUP_LINK)) == 0
                            || read_u8(target.wrapping_add(TARGET_STATE_FLAGS)) & 4 == 0;
                        write_u32(
                            this.wrapping_add(TASK_PATROL_TIMER),
                            if no_target_pressure {
                                TWO_SECONDS_BITS
                            } else {
                                ONE_SECOND_BITS
                            },
                        );
                    } else {
                        write_u32(this.wrapping_add(TASK_PATROL_TIMER), FIVE_SECONDS_BITS);
                    }

                    let random = lf_checker_rt::callee_cdecl!(CALL_092, u32,) as i32 as f32;
                    let factor = add_f32(
                        mul_f32(
                            mul_f32(random, global_f32(G_RANDOM_SCALE)),
                            global_f32(G_RANDOM_MEDIUM),
                        ),
                        global_f32(G_RANDOM_BIAS),
                    );
                    let new_timer = mul_f32(
                        factor,
                        read_f32(this.wrapping_add(TASK_PATROL_TIMER)),
                    );
                    write_f32(this.wrapping_add(TASK_PATROL_TIMER), new_timer);

                    if (lf_checker_rt::callee_cdecl!(CALL_093, u32, ped) & 0xff) != 0 {
                        let random = lf_checker_rt::callee_cdecl!(CALL_094, u32,) as i32 as f32;
                        let scaled = mul_f32(random, global_f32(G_RANDOM_SCALE));
                        let long_delay = add_f32(
                            add_f32(
                                mul_f32(scaled, global_f32(G_RANDOM_LARGE)),
                                global_f32(G_RANDOM_BASE),
                            ),
                            read_f32(this.wrapping_add(TASK_PATROL_TIMER)),
                        );
                        write_f32(this.wrapping_add(TASK_PATROL_TIMER), long_delay);
                    }
                }
            }
        }

        if target != 0 {
            let current_kind = subtask_type(CALL_095, this);
            let action_kind = current_kind == TYPE_DRAWING
                || (current_kind != TYPE_DRAWING
                    && subtask_type(CALL_096, this) == TYPE_ARMED
                    && (lf_checker_rt::callee_cdecl!(CALL_097, u32, ped, target) & 0xff) != 0);
            if action_kind
                && read_f32(this.wrapping_add(TASK_COOLDOWN_B)) <= FRAME_ZERO
            {
                let response = lf_checker_rt::callee_thiscall!(CALL_098, u32, this, ped, 0, 1);
                if response != 0 {
                    if ped_predicate(CALL_099, subtask, ped) {
                        return response;
                    }
                    let _: u32 = slot_zero(CALL_100, response, 1);
                }
            }
        }

        if read_u32(ped.wrapping_add(PED_GROUP_VALUE)) != 0 {
            write_u32(
                ped.wrapping_add(PED_MODE_FLAGS),
                read_u32(ped.wrapping_add(PED_MODE_FLAGS)) | 0x800,
            );
        }

        let first_kind = subtask_type(CALL_101, this);
        let task_type_gate = if first_kind == TYPE_THREATENED {
            true
        } else {
            let second_kind = subtask_type(CALL_102, this);
            if second_kind == TYPE_IDLE {
                true
            } else {
                let third_kind = subtask_type(CALL_103, this);
                if third_kind == TYPE_DRAWING {
                    true
                } else {
                    let fourth_kind = subtask_type(CALL_104, this);
                    if fourth_kind == TYPE_SEARCH {
                        true
                    } else {
                        subtask_type(CALL_105, this) == TYPE_GANG
                            || (lf_checker_rt::callee_thiscall!(
                                CALL_106,
                                u32,
                                group_decision,
                                0,
                                0
                            ) & 0xff)
                                != 0
                    }
                }
            }
        };

        if task_type_gate {
            if subtask_type(CALL_109, this) == TYPE_THREATENED
                && (lf_checker_rt::callee_thiscall!(CALL_110, u32, group_decision, 0, 0) & 0xff) != 0
                && ped_predicate(CALL_111, subtask, ped)
            {
                let can_draw = (read_u8(ped.wrapping_add(PED_MODE_FLAGS)) & 4 == 0
                    || read_u32(ped.wrapping_add(PED_GROUP_LINK)) == 0)
                    && (lf_checker_rt::callee_thiscall!(
                        CALL_112,
                        u32,
                        target.wrapping_add(0x2b0),
                        1
                    ) & 0xff)
                        != 0
                    && lf_checker_rt::callee_thiscall!(CALL_113, u32, group_decision, target) != 2;
                if can_draw {
                    if read_u32(ped_group.wrapping_add(GROUP_SIZE)) == GROUP_KIND {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            CALL_114,
                            u32,
                            ped.wrapping_add(PED_SPEECH),
                            lf_checker_rt::relocated(SPEECH_DRAW_GUN),
                            1,
                            0,
                            0,
                            u32::MAX,
                            0,
                            0,
                            ONE_SECOND_BITS,
                            0,
                            0
                        );
                    }
                    write_u32(subtask.wrapping_add(0x14), 4);
                    let ped_position = read_u32(ped.wrapping_add(PED_POSITION));
                    let target_position = read_u32(target.wrapping_add(PED_POSITION));
                    let distance_squared = vector_distance_squared(ped_position, target_position);
                    if distance_squared < global_f32(G_SHORT_RANGE) {
                        write_u32(subtask.wrapping_add(SUBTASK_KIND), TYPE_ARMED);
                        return lf_checker_rt::callee_thiscall!(CALL_115, u32, this, ped, 0, 0);
                    }
                    write_u32(subtask.wrapping_add(SUBTASK_KIND), TYPE_DRAWING);
                }
                return lf_checker_rt::callee_thiscall!(CALL_116, u32, this, ped, 0, 0);
            }
        } else if ped_predicate(CALL_107, subtask, ped) {
            return lf_checker_rt::callee_thiscall!(CALL_108, u32, this, TYPE_THREATENED, ped);
        }

        let task_state = read_u32(this.wrapping_add(TASK_STATE_FLAGS));
        if task_state & 2 != 0 {
            write_u32(this.wrapping_add(TASK_STATE_FLAGS), task_state & !2);
            let active_kind = lf_checker_rt::callee_thiscall!(CALL_117, u32, subtask, ped);
            return lf_checker_rt::callee_thiscall!(CALL_118, u32, this, active_kind);
        }
        if task_state & 4 != 0 {
            write_u32(this.wrapping_add(TASK_STATE_FLAGS), task_state & !4);
            return lf_checker_rt::callee_thiscall!(CALL_119, u32, this, ped);
        }
        subtask
    }
});
