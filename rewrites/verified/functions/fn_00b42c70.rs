// original: 0x00B42C70 physical_frag_update

/// Register a physical fragment for the relevant spatial or task update.
///
/// `fragment` is an entity-like object. Its model index is the signed word at
/// `+0x2e`, its category and update flags share the word at `+0x28`, and its
/// model table entry points to the model interface. The function rejects
/// unsupported categories, missing model data, excluded model flags, and a
/// model whose virtual status query returns 4. It then reads two virtual
/// positions and applies category-specific distance and state gates. The
/// optional player-area helper and the mass query are called only on their
/// corresponding branches; the final registration helper runs when all gates
/// pass. The function has no meaningful return value; callers use its side
/// effects in the callees.
///
/// Float comparisons preserve the original COMISS unordered behavior: JBE is
/// true for an unordered comparison, while JA is false. Float subtraction is
/// kept in the original `second - first` order. Model indices are signed;
/// flag and category fields are treated as unsigned bit fields.
lf_checker_rt::export!(cdecl, rw_00b42c70(fragment: u32) -> u32 {
    unsafe {
        const CATEGORY_FLAGS: u32 = 0x28;
        const MODEL_INDEX: u32 = 0x2e;
        const HAS_MODEL: u32 = 0x38;
        const MODEL_FLAGS: u32 = 0x40;
        const POSITION_A_SLOT: u32 = 0x60;
        const POSITION_B_SLOT: u32 = 0x64;
        const MODEL_STATUS_SLOT: u32 = 0x0c;
        const MASS_SLOT: u32 = 0x24;
        const OBJECT_BIT_FLAGS: u32 = 0x24;
        const UPDATE_MODE: u32 = 0x210;
        const FRAGMENT_KIND: u32 = 0x1e2;
        const FRAGMENT_STATE: u32 = 0x22a;
        const NESTED_PATH: u32 = 0x20;
        const MODEL_TABLE_VA: u32 = 0x0129_5cd8;
        const PLAYER_QUERY_ENABLED_VA: u32 = 0x011d_6fd4;
        const PLAYER_MODEL_INDEX_VA: u32 = 0x012f_a620;
        const AREA_QUERY_SECOND_VA: u32 = 0x0104_59f0;
        const AREA_QUERY_THIRD_VA: u32 = 0x0104_5a00;
        const QUERY_CALL: u32 = 5;
        const MASS_OBJECT_CALL: u32 = 6;
        const REGISTER_CALL: u32 = 7;

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
        unsafe fn read_f32(address: u32) -> f32 {
            unsafe { f32::from_bits(read_u32(address)) }
        }
        #[inline(always)]
        fn sub_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn jbe(left: f32, right: f32) -> bool {
            left <= right || left.is_nan() || right.is_nan()
        }
        #[inline(always)]
        fn ja(left: f32, right: f32) -> bool {
            left > right
        }
        #[inline(always)]
        unsafe fn vcall_u8(object: u32, slot: u32) -> u8 {
            unsafe {
                let table = read_u32(object);
                let entry = read_u32(table + slot);
                let method: extern "thiscall" fn(u32) -> u8 =
                    core::mem::transmute(entry as usize);
                method(object)
            }
        }
        #[inline(always)]
        unsafe fn vcall_u32(object: u32, slot: u32) -> u32 {
            unsafe {
                let table = read_u32(object);
                let entry = read_u32(table + slot);
                let method: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(entry as usize);
                method(object)
            }
        }
        #[inline(always)]
        unsafe fn vcall_f32(object: u32, slot: u32) -> f32 {
            unsafe {
                let table = read_u32(object);
                let entry = read_u32(table + slot);
                let method: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(entry as usize);
                method(object)
            }
        }

        let packed_kind = (read_u32(fragment + CATEGORY_FLAGS) >> 6) & 0x0f;
        if packed_kind != 2 && packed_kind != 4 {
            return 0;
        }
        if read_u32(fragment + HAS_MODEL) == 0 {
            return 0;
        }

        let model_index = i32::from(read_u16(fragment + MODEL_INDEX) as i16);
        let model_table = lf_checker_rt::global::<u32>(MODEL_TABLE_VA) as u32;
        let model_entry = model_table.wrapping_add((model_index as u32).wrapping_mul(4));
        let model = read_u32(model_entry);
        let model_flags = read_u32(model + MODEL_FLAGS);
        if model_flags & 0x0008_0000 != 0
            || (model_flags & 0x0040_0000 != 0 && model_flags & 0x0004_0000 == 0)
        {
            return 0;
        }
        if vcall_u8(model, MODEL_STATUS_SLOT) == 4 {
            return 0;
        }

        let first_position = vcall_u32(fragment, POSITION_A_SLOT);
        let second_position = vcall_u32(fragment, POSITION_B_SLOT);
        let delta_x = sub_f32(read_f32(second_position), read_f32(first_position));
        let delta_y = sub_f32(read_f32(second_position + 4), read_f32(first_position + 4));
        let delta_z = sub_f32(read_f32(second_position + 8), read_f32(first_position + 8));

        if read_u32(fragment + CATEGORY_FLAGS) & 0x0000_03c0 != 0x0000_0100 {
            let minimum_motion = read_f32(lf_checker_rt::global::<f32>(0x00fe_87e8) as u32);
            if jbe(minimum_motion, delta_x)
                || jbe(minimum_motion, delta_y)
                || jbe(minimum_motion, delta_z)
            {
                let _ = lf_checker_rt::callee_cdecl!(REGISTER_CALL, u32, fragment, model);
            }
            return 0;
        }

        if read_u8(fragment + UPDATE_MODE) & 1 != 0 {
            return 0;
        }
        if read_u8(fragment + FRAGMENT_KIND) & 0x0f >= 2 {
            return 0;
        }

        let state = read_u8(fragment + FRAGMENT_STATE);
        let player_query_matches = read_u32(
            lf_checker_rt::global::<u32>(PLAYER_QUERY_ENABLED_VA) as u32,
        ) == 1
            && model_index as u32
                == read_u32(lf_checker_rt::global::<u32>(PLAYER_MODEL_INDEX_VA) as u32);
        let area_query_result = if player_query_matches {
            let nested_path = read_u32(fragment + NESTED_PATH);
            let query_context = if nested_path != 0 {
                nested_path.wrapping_add(0x30)
            } else {
                fragment.wrapping_add(0x10)
            };
            lf_checker_rt::callee_cdecl!(
                QUERY_CALL,
                u8,
                query_context,
                lf_checker_rt::relocated(AREA_QUERY_SECOND_VA),
                lf_checker_rt::relocated(AREA_QUERY_THIRD_VA),
            ) != 0
        } else {
            false
        };

        if read_u32(fragment + OBJECT_BIT_FLAGS) & 4 != 0 && state != 2 && !area_query_result {
            return 0;
        }

        let should_register = if state == 4 {
            let mass_object = lf_checker_rt::callee_thiscall!(MASS_OBJECT_CALL, u32, fragment);
            let mass = vcall_f32(mass_object, MASS_SLOT);
            let required_mass = read_f32(lf_checker_rt::global::<f32>(0x00fe_8c58) as u32);
            if jbe(required_mass, mass) {
                true
            } else {
                let minimum_axis_motion =
                    read_f32(lf_checker_rt::global::<f32>(0x00fe_8ae0) as u32);
                !(ja(minimum_axis_motion, delta_x)
                    || ja(minimum_axis_motion, delta_y)
                    || ja(minimum_axis_motion, delta_z))
            }
        } else {
            let minimum_motion = read_f32(lf_checker_rt::global::<f32>(0x00fe_87e8) as u32);
            jbe(minimum_motion, delta_x)
                || jbe(minimum_motion, delta_y)
                || jbe(minimum_motion, delta_z)
        };

        if should_register {
            let _ = lf_checker_rt::callee_cdecl!(REGISTER_CALL, u32, fragment, model);
        }
        0
    }
});
